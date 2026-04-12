mod command;
mod state;
mod sync;
mod thread;

use {
  self::{
    command::Command,
    state::SharedInternal,
  },
  crate::window::{
    command::{
      CommandEnvelope,
      CommandId,
      CommandResponse,
    },
    sync::{
      AcknowledgementToken,
      EventEnvelope,
      ResponseEnvelope,
      WindowToMain,
    },
  },
  ::win64::Handle,
  crossbeam_channel::{
    Receiver,
    Sender,
    TryRecvError,
  },
  crossbeam_queue::SegQueue,
  std::{
    collections::HashMap,
    sync::{
      Arc,
      Mutex,
    },
  },
  ventana_hal::{
    self,
    error::RequestError,
    event::{
      Event,
      WindowEvent,
    },
    input::mouse::MouseButton,
    keyboard::{
      Code,
      KeyState,
    },
    monitor::BackendMonitor,
    settings::WindowSettings,
    types::Flow,
    window::{
      BackendWindow,
      WindowId,
    },
  },
  win64::prelude::*,
};

// No `Arc` necessary for fields other than `shared` as this will be inside an `Arc<dyn BackendWindow>`
pub struct Win32Window {
  hwnd: Window,

  msg_rx: Receiver<WindowToMain>,
  cmd_tx: Sender<CommandEnvelope>,

  event_backlog: SegQueue<EventEnvelope>,
  pending_ack: Mutex<Option<AcknowledgementToken>>,

  command_responses: Mutex<HashMap<CommandId, CommandResponse>>,

  shared: Arc<SharedInternal>, // This is Arc so that it can be shared with the Window thread
}

impl Drop for Win32Window {
  fn drop(&mut self) {
    self.acknowledge_previous_event();
    self.send_command(Command::Destroy);
    log::trace!("Destroyed window");
  }
}

impl Win32Window {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    set_process_dpi_awareness(DPIAwarenessContext::PerMonitorAwareV2);

    let (msg_tx, msg_rx) = crossbeam_channel::unbounded();
    let (cmd_tx, cmd_rx) = crossbeam_channel::unbounded();

    // TODO: Rewrite this so that settings doesn't need to be cloned
    let shared = SharedInternal::new(settings.clone(), msg_tx, cmd_rx);
    let hwnd = shared.spawn_thread(settings)?;

    Ok(Self {
      hwnd,
      msg_rx,
      cmd_tx,
      event_backlog: Default::default(),
      pending_ack: Mutex::new(None),
      command_responses: Default::default(),
      shared,
    })
  }

  fn send_command(&self, command: Command) -> Option<CommandResponse> {
    let envelope: CommandEnvelope = command.into();
    let id = envelope.id;
    self.cmd_tx.try_send(envelope).ok()?;

    // Pump events while waiting to find response and queue them up in the backlog to be processed properly later
    loop {
      if let Some(response) = self.command_responses.lock().ok()?.remove(&id) {
        return match response {
          CommandResponse::Empty => None,
          _ => Some(response),
        };
      }

      match self.msg_rx.try_recv().ok()? {
        WindowToMain::Event(EventEnvelope { event, ack }) => {
          self.event_backlog.push(EventEnvelope { event, ack: None });
          if let Some(ack) = ack {
            ack.send();
          }
        },
        WindowToMain::CommandResponse(ResponseEnvelope { id, response }) => {
          self.command_responses.lock().ok()?.insert(id, response);
        },
      }
    }
  }

  fn acknowledge_previous_event(&self) {
    if let Some(token) = self.pending_ack.lock().unwrap().take() {
      token.send();
    }
  }

  fn receive_event(&self) -> Option<WindowToMain> {
    let flow = self.shared.state_lock().flow;
    match flow {
      Flow::Wait => self.msg_rx.recv().ok(),
      Flow::Poll => match self.msg_rx.try_recv() {
        Ok(event) => Some(event),
        Err(TryRecvError::Empty) => Some(WindowToMain::Event(EventEnvelope::empty())),
        Err(TryRecvError::Disconnected) => None,
      },
    }
  }

  fn pump_event_and_block(&self) {
    match self.receive_event() {
      None => (),
      Some(WindowToMain::Event(event)) => {
        self.event_backlog.push(event);
      },
      Some(WindowToMain::CommandResponse(ResponseEnvelope { id, response })) => {
        self.command_responses.lock().unwrap().insert(id, response);
      },
    }
  }
}

impl BackendWindow for Win32Window {
  fn id(&self) -> WindowId {
    WindowId::from_raw(self.hwnd.to_ptr() as usize)
  }

  fn raw_window_handle(&self) -> ventana_hal::raw_window_handle::RawWindowHandle {
    #[cfg(raw_window_handle_v5)]
    {
      let mut handle = ventana_hal::raw_window_handle::Win32WindowHandle::empty();
      handle.hwnd = self.hwnd.to_ptr();
      handle.hinstance = self.hwnd.instance().to_ptr();
      handle.into()
    }

    #[cfg(raw_window_handle_v6)]
    {
      let mut handle = ventana_hal::raw_window_handle::Win32WindowHandle::new(
        std::num::NonZeroIsize::new(self.hwnd.to_ptr() as isize).expect("window handle should not be zero"),
      );

      let hinstance = std::num::NonZeroIsize::new(self.hwnd.instance().to_ptr() as isize)
        .expect("instance handle should not be zero");
      handle.hinstance = Some(hinstance);
      handle.into()
    }
  }

  fn raw_display_handle(&self) -> ventana_hal::raw_window_handle::RawDisplayHandle {
    #[cfg(raw_window_handle_v5)]
    {
      let handle = ventana_hal::raw_window_handle::WindowsDisplayHandle::empty();
      handle.into()
    }

    #[cfg(raw_window_handle_v6)]
    {
      let handle = ventana_hal::raw_window_handle::WindowsDisplayHandle::new();
      handle.into()
    }
  }

  fn next(&self) -> Option<Event> {
    self.acknowledge_previous_event();

    if self.shared.should_close() {
      return None;
    }

    loop {
      if let Some(EventEnvelope { event, ack }) = self.event_backlog.pop() {
        if let Some(ack) = ack {
          self.pending_ack.lock().unwrap().replace(ack);
        }

        if let Event::Window(WindowEvent::CloseRequest) = event {
          let x = self.shared.state_lock().close_on_x;
          if x {
            self.close();
          }
        }

        return Some(event);
      }

      self.pump_event_and_block();
    }
  }

  fn monitor(&self) -> Arc<dyn BackendMonitor> {
    todo!()
  }

  fn close(&self) {
    self.shared.state_lock().is_running = false;
  }

  fn is_closing(&self) -> bool {
    self.shared.should_close()
  }

  fn request_redraw(&self) {
    self.send_command(Command::Redraw);
  }

  fn title(&self) -> String {
    let Some(CommandResponse::GetWindowText(text)) = self.send_command(Command::GetWindowText) else {
      return String::new();
    };
    text
  }

  fn scale_factor(&self) -> f64 {
    self.hwnd.scale_factor()
  }

  fn inner_size(&self) -> PhysicalSize<u32> {
    self.hwnd.client_size()
  }

  fn outer_size(&self) -> PhysicalSize<u32> {
    self.hwnd.window_size()
  }

  fn inner_position(&self) -> PhysicalPosition<i32> {
    self.hwnd.client_position()
  }

  fn outer_position(&self) -> PhysicalPosition<i32> {
    self.hwnd.window_position()
  }

  fn key(&self, keycode: Code) -> KeyState {
    log::debug!("Checking {keycode:?}...");
    todo!()
  }

  fn mouse(&self, button: MouseButton) -> KeyState {
    log::debug!("Checking {button:?}...");
    todo!()
  }

  fn shift_key(&self) -> KeyState {
    todo!()
  }

  fn ctrl_key(&self) -> KeyState {
    todo!()
  }

  fn alt_key(&self) -> KeyState {
    todo!()
  }

  fn super_key(&self) -> KeyState {
    todo!()
  }
}
