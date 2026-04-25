#![cfg(windows)]

mod command;
mod state;
mod thread;

use {
  self::{
    command::Command,
    state::SharedInternal,
    thread::Win32ThreadHandler,
  },
  crate::window::command::{
    CommandResponse,
    CreateInfo,
  },
  ::win64::Handle,
  std::sync::Arc,
  threadloop::{
    ThreadLoop,
    message::ClientToServer,
  },
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
    pointer::{
      ButtonState,
      mouse::MouseButton,
    },
    settings::WindowSettings,
    types::Flow,
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

// struct Win32Client {
//   hwnd: Option<Window>,
//   shared: Arc<SharedInternal>,
// }

// impl Client for Win32Client {
//   fn destroy(&self, client: &ClientWrapper<Self::Command, Self::ThreadResponse, Self>) -> Result<(), RequestError> {
//     self.shared.set_ready(false);
//     log::trace!("Sending Command::Destroy...");
//     client.send_command(Command::Destroy, true);
//     Ok(())
//   }
// }

// No `Arc` necessary for fields other than `shared` as this will be inside an `Arc<dyn BackendWindow>`
pub struct Win32Window {
  hwnd: Window,
  flow: Flow,
  shared: Arc<SharedInternal>,
  thread: Arc<ThreadLoop<Win32ThreadHandler>>,
}

impl Drop for Win32Window {
  fn drop(&mut self) {
    log::trace!("Dropping window");
    // self
    //   .thread
    //   .join()
    //   .expect("panicked when attempting to join Window thread");
  }
}

impl Win32Window {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    set_process_dpi_awareness(DPIAwarenessContext::PerMonitorAwareV2);

    log::trace!("Creating ThreadLoop");
    let thread = ThreadLoop::new(Arc::new(Win32ThreadHandler::new())).request_error()?;

    log::trace!("Sending Command::CreateWindow...");

    let flow = settings.flow;
    let shared = SharedInternal::new(settings.clone());
    let Some(CommandResponse::CreateWindow(hwnd)) = thread
      .send_request(ClientToServer::request(Command::CreateWindow(CreateInfo {
        shared: shared.clone(),
        settings,
      })))
      .unwrap()
    else {
      unreachable!("Command::CreateWindow should always return CommandResponse::CreateWindow")
    };

    log::trace!("Received window handle from window thread");

    Ok(Self {
      hwnd,
      flow,
      shared,
      thread,
    })
  }

  fn hwnd(&self) -> Window {
    self.hwnd
  }
}

impl BackendWindow for Win32Window {
  fn id(&self) -> WindowId {
    WindowId::from_raw(self.hwnd().to_ptr() as usize)
  }

  fn raw_window_handle(&self) -> RawWindowHandle {
    let mut handle = Win32WindowHandle::new(
      std::num::NonZeroIsize::new(self.hwnd().to_ptr() as isize).expect("window handle should not be zero"),
    );

    let hinstance = std::num::NonZeroIsize::new(self.hwnd().instance().to_ptr() as isize)
      .expect("instance handle should not be zero");
    handle.hinstance = Some(hinstance);
    handle.into()
  }

  fn raw_display_handle(&self) -> RawDisplayHandle {
    let handle = WindowsDisplayHandle::new();
    handle.into()
  }

  fn next(&self) -> Option<Event> {
    let event = self.thread.next_event(matches!(self.flow, Flow::Wait))?;

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
    self.thread.send_request(ClientToServer::stop());
  }

  fn is_closing(&self) -> bool {
    todo!()
    // self.stop_signal.should_stop()
  }

  fn request_redraw(&self) {
    log::trace!("Sending Command::Redraw...");
    self.thread.send_request(ClientToServer::request(Command::Redraw));
  }

  fn title(&self) -> String {
    log::trace!("Sending Command::GetWindowText...");
    let Ok(Some(CommandResponse::GetWindowText(text))) = self
      .thread
      .send_request(ClientToServer::request(Command::GetWindowText))
    else {
      return String::new();
    };
    text
  }

  fn set_title(&self, title: String) {
    log::trace!("Sending Command::SetWindowText...");
    self
      .thread
      .send_request(ClientToServer::request(Command::SetWindowText(title)));
  }

  fn scale_factor(&self) -> f64 {
    self.hwnd().scale_factor()
  }

  fn inner_size(&self) -> PhysicalSize<u32> {
    self.hwnd().client_size()
  }

  fn outer_size(&self) -> PhysicalSize<u32> {
    self.hwnd().window_size()
  }

  fn inner_position(&self) -> PhysicalPosition<i32> {
    self.hwnd().client_position()
  }

  fn outer_position(&self) -> PhysicalPosition<i32> {
    self.hwnd().window_position()
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
