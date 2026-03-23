mod create_info;
mod sync;
mod thread;

use {
  self::{
    create_info::CreateInfo,
    sync::SyncData,
    thread::WindowThread,
  },
  crate::state::State,
  ::win64::Handle,
  std::sync::{
    Arc,
    Mutex,
    RwLock,
  },
  ventana_hal::{
    dpi::{
      Position,
      Size,
    },
    error::{
      MapToOSError,
      RequestError,
    },
    event::Event,
    input::mouse::MouseButton,
    keyboard::{
      Code,
      KeyState,
    },
    settings::WindowSettings,
    window::{
      BackendWindow,
      WindowId,
    },
  },
  win64::prelude::*,
};

pub struct Win32Window {
  hwnd: Window,
  state: RwLock<State>,
  thread: WindowThread,
}

impl Win32Window {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    win64::set_process_dpi_awareness(win64::DPIAwarenessContext::PerMonitorAwareV2);

    let create_info = CreateInfo {
      settings: settings.clone(),
      message: Arc::new(Mutex::new(None)),
      sync: SyncData::new(),
    };

    let (window_sender, window_receiver) = std::sync::mpsc::sync_channel(0);
    let thread = WindowThread::spawn(window_sender, create_info)?;

    log::trace!("Waiting to receive window handle back from window thread");

    let hwnd = window_receiver
      .recv()
      .expect("Failed to receive window back from window thread");

    log::trace!("Received window handle from window thread: `{hwnd:?}`");

    Ok(Arc::new(Self {
      hwnd,
      state: RwLock::new(State::new()),
      thread,
    }))
  }
}

impl BackendWindow for Win32Window {
  fn id(&self) -> WindowId {
    WindowId::from_raw(self.hwnd.to_ptr() as usize)
  }

  fn next(&self) -> Option<Event> {
    None
  }

  fn title(&self) -> String {
    self.hwnd.get_window_text().unwrap()
  }

  fn inner_size(&self) -> Size {
    self.hwnd.inner_size()
  }

  fn outer_size(&self) -> Size {
    self.hwnd.outer_size()
  }

  fn inner_position(&self) -> Position {
    self.hwnd.inner_position()
  }

  fn outer_position(&self) -> Position {
    self.hwnd.outer_position()
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
