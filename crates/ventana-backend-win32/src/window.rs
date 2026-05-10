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
  crate::{
    Win32,
    monitor::Win32Monitor,
    window::command::CreateInfo,
  },
  ::win64::Handle,
  std::sync::{
    Arc,
    atomic::{
      AtomicBool,
      Ordering,
    },
  },
  threadloop::{
    NextEventError,
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
      KeyEvent,
      WindowEvent,
    },
    input::key::PhysicalKey,
    keyboard::{
      Code,
      KeyState,
    },
    monitor::BackendMonitor,
    pointer::{
      ButtonState,
      PointerButton,
      PointerEvent,
    },
    settings::WindowSettings,
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
  // flow: Flow,
  should_quit: Arc<AtomicBool>,
  shared: Arc<SharedInternal>,
  thread: Arc<ThreadLoop<Win32ThreadHandler>>, // held as a ptr because it's chonky
}

impl Drop for Win32Window {
  fn drop(&mut self) {
    log::trace!("Dropping window");
  }
}

impl Win32Window {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    set_process_dpi_awareness(DPIAwarenessContext::PerMonitorAwareV2);

    // let flow = settings.flow;
    let shared = SharedInternal::new(settings.clone());

    log::trace!("Creating ThreadLoop");

    let thread =
      ThreadLoop::new(Arc::new(Win32ThreadHandler::new()), CreateInfo { shared: shared.clone(), settings })
        .request_error()?;

    log::trace!("Sending Command::CreateWindow...");

    let hwnd = thread.start().request_error()?;

    log::trace!("Received window handle from window thread");

    Ok(Self { hwnd, should_quit: Arc::new(AtomicBool::new(false)), shared, thread })
  }

  fn hwnd(&self) -> Window {
    self.hwnd
  }

  fn next_event<const SHOULD_WAIT: bool>(&self) -> Option<Event> {
    if self.is_closing() {
      // TODO: reset flag and set active in case we start looping again later
      // self.should_quit.store(false, Ordering::Release);
      self.thread.set_inactive();
      return None;
    }

    let event = match self.thread.next_event(SHOULD_WAIT) {
      Ok(event) => event,
      Err(NextEventError::Empty) => Event::None,
      Err(NextEventError::Disconnected) => return None,
    };

    match event {
      Event::Window(WindowEvent::CloseRequest) => {
        let x = self.shared.close_on_x;
        if x {
          self.close();
        }
      },
      Event::Window(WindowEvent::Keyboard(KeyEvent { state, code: PhysicalKey::Code(code), .. })) => {
        Win32::input_mut().update_key(code, state);
      },

      Event::Window(WindowEvent::Pointer(PointerEvent::Button { state, button, .. })) => {
        Win32::input_mut().update_pointer(button, state);
      },
      _ => (),
    }

    Some(event)
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
    self.next_event::<true>()
  }

  fn try_next(&self) -> Option<Event> {
    self.next_event::<false>()
  }

  fn monitor(&self) -> Arc<dyn BackendMonitor> {
    Arc::new(Win32Monitor(self.hwnd.monitor()))
  }

  fn close(&self) {
    self.should_quit.store(true, Ordering::Release);
  }

  fn is_closing(&self) -> bool {
    self.should_quit.load(Ordering::Acquire)
  }

  // this should be changed to activate a flag to avoid excessive redraws
  fn request_redraw(&self) {
    // log::trace!("Sending Command::Redraw...");
    self.thread.proxy().send_request(ClientToServer::request(Command::Redraw)).unwrap();
  }

  fn title(&self) -> String {
    // log::trace!("Sending Command::GetWindowText...");
    // match self.thread.send_and_wait(ClientToServer::request(Command::GetWindowText)) {
    //   Ok(Some(CommandResponse::GetWindowText(text))) => text,
    //   _ => String::new(),
    // }
    self.shared.title().clone()
  }

  fn set_title(&self, title: String) {
    // log::trace!("Sending Command::SetWindowText...");
    self.thread.proxy().send_request(ClientToServer::request(Command::SetWindowText(title))).unwrap();
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

  fn key(&self, code: Code) -> KeyState {
    Win32::input().key(code)
  }

  fn pointer(&self, button: PointerButton) -> ButtonState {
    Win32::input().pointer(button)
  }

  fn shift_key(&self) -> KeyState {
    Win32::input().shift_key
  }

  fn ctrl_key(&self) -> KeyState {
    Win32::input().ctrl_key
  }

  fn alt_key(&self) -> KeyState {
    Win32::input().alt_key
  }

  fn meta_key(&self) -> KeyState {
    Win32::input().meta_key
  }
}
