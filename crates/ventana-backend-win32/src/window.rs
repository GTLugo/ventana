#![cfg(target_os = "windows")]

mod command;
mod state;
mod thread;

use {
  self::{
    command::Command,
    state::SharedInternal,
    thread::Procedure,
  },
  crate::window::command::CommandResponse,
  ::win64::Handle,
  std::sync::Arc,
  ventana_hal::{
    error::{
      MapToOSError,
      RequestError,
    },
    event::{
      Event,
      WindowEvent,
    },
    keyboard::{
      Code,
      KeyState,
    },
    monitor::BackendMonitor,
    os_error_fmt,
    pointer::{
      ButtonState,
      mouse::MouseButton,
    },
    settings::WindowSettings,
    thread::{
      ThreadLoop,
      server::Server,
      signal::StopSignal,
    },
    window::{
      BackendWindow,
      WindowId,
    },
  },
  win64::{
    prelude::*,
    raw_window_handle::{
      RawDisplayHandle,
      RawWindowHandle,
      Win32WindowHandle,
      WindowsDisplayHandle,
    },
  },
};

// No `Arc` necessary for fields other than `shared` as this will be inside an `Arc<dyn BackendWindow>`
pub struct Win32Window {
  hwnd: Window,

  thread: ThreadLoop<Window, Command, CommandResponse>,
  stop_signal: StopSignal,

  shared: Arc<SharedInternal>, // This is Arc so that it can be shared with the Window thread
}

impl Drop for Win32Window {
  fn drop(&mut self) {
    log::trace!("Dropping Win32Window");
    self
      .thread
      .join()
      .expect("panicked when attempting to join Window thread");
    log::trace!("Destroyed window");
  }
}

impl Win32Window {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    set_process_dpi_awareness(DPIAwarenessContext::PerMonitorAwareV2);

    let shared = SharedInternal::new(settings.clone());
    let mut thread = ThreadLoop::new(
      settings.flow,
      |client| {
        // On drop
        client.send_command(Command::Destroy);
      },
      |window: &Window| {
        // On wake
        const WAKE_MESSAGE: u32 = Message::APP + 67;
        window
          .post_message(Message::App(AppMessage::empty(WAKE_MESSAGE)))
          .unwrap();
      },
    );
    let stop_signal = thread.loop_signal();
    let hwnd = Self::start_thread(&mut thread, shared.clone(), settings)?;

    Ok(Self {
      hwnd,
      thread,
      shared,
      stop_signal,
    })
  }

  fn start_thread(
    thread: &mut ThreadLoop<Window, Command, CommandResponse>,
    shared: Arc<SharedInternal>,
    settings: WindowSettings,
  ) -> Result<Window, RequestError> {
    let (window_tx, window_rx) = crossbeam_channel::bounded(0);
    thread.run(move |server| {
      log::trace!("Starting window thread");

      let window = Self::create_window(server, shared.clone(), settings)?;
      window_tx
        .send(window)
        .map_err(|e| os_error_fmt!("failed to send window handle: `{e}`"))?;

      shared.set_ready(true);

      log::trace!("Window is ready; entering message loop");

      MessageLoop::new().run();

      log::trace!("Joining main thread");

      Ok(())
    })?;

    log::trace!("Waiting to receive window handle back from window thread");

    let hwnd = window_rx
      .recv()
      .expect("Failed to receive window back from window thread");

    thread.set_window(hwnd);

    log::trace!("Received window handle from window thread: `{hwnd:?}`");

    Ok(hwnd)
  }

  fn create_window(
    server: Arc<Server<Command, CommandResponse>>,
    internal: Arc<SharedInternal>,
    settings: WindowSettings,
  ) -> Result<Window, RequestError> {
    let class = {
      let mut class = WindowClass::builder()
        .with_name("Window Class")
        .with_style(WindowClassStyle::DoubleClicks);
      if let Some(color) = settings.clear_color {
        class = class.with_background_brush(Brush::solid(color));
      }
      class
    }
    .register()
    .map_to_os_err()?;
    log::debug!("{settings:?}");
    let hwnd = class
      .create_window()
      .with_procedure(Procedure { server, internal })
      .with_name(settings.title.clone())
      .with_style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
      .with_position(settings.position)
      .with_size(Some(settings.size))
      .create()
      .map_to_os_err()?;
    Ok(hwnd)
  }
}

impl BackendWindow for Win32Window {
  fn id(&self) -> WindowId {
    WindowId::from_raw(self.hwnd.to_ptr() as usize)
  }

  fn raw_window_handle(&self) -> RawWindowHandle {
    let mut handle = Win32WindowHandle::new(
      std::num::NonZeroIsize::new(self.hwnd.to_ptr() as isize).expect("window handle should not be zero"),
    );

    let hinstance =
      std::num::NonZeroIsize::new(self.hwnd.instance().to_ptr() as isize).expect("instance handle should not be zero");
    handle.hinstance = Some(hinstance);
    handle.into()
  }

  fn raw_display_handle(&self) -> RawDisplayHandle {
    let handle = WindowsDisplayHandle::new();
    handle.into()
  }

  fn next(&self) -> Option<Event> {
    let event = self.thread.next_event()?;

    if let Event::Window(WindowEvent::CloseRequest) = event {
      let x = self.shared.state_lock().close_on_x;
      if x {
        self.close();
      }
    }

    Some(event)
  }

  fn monitor(&self) -> Arc<dyn BackendMonitor> {
    todo!()
  }

  fn close(&self) {
    self.stop_signal.stop();
  }

  fn is_closing(&self) -> bool {
    self.stop_signal.should_stop()
  }

  fn request_redraw(&self) {
    self.thread.send_command(Command::Redraw);
  }

  fn title(&self) -> String {
    let Some(CommandResponse::GetWindowText(text)) = self.thread.send_command(Command::GetWindowText) else {
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

  fn mouse(&self, button: MouseButton) -> ButtonState {
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
