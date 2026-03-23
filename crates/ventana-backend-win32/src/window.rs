use {
  crate::state::State,
  ::win64::Handle,
  std::sync::{
    Arc,
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
  state: Arc<RwLock<State>>,
}

impl Win32Window {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    win64::set_process_dpi_awareness(win64::DPIAwarenessContext::PerMonitorAwareV2);

    let class = WindowClass::builder().name("Window Class").register().map_to_os_err()?;
    let hwnd = class
      .window_builder()
      .procedure(Internal)
      .name(settings.title.clone())
      .position(settings.position)
      .size(Some(settings.size))
      .create()
      .map_to_os_err()?;

    // MessagePump::default().with_mode(PollingMode::Poll).run();

    Ok(Arc::new(Self {
      hwnd,
      state: Arc::new(RwLock::new(State::new())),
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
    todo!()
  }

  fn outer_size(&self) -> Size {
    todo!()
  }

  fn inner_position(&self) -> Position {
    todo!()
  }

  fn outer_position(&self) -> Position {
    todo!()
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

struct Internal;

impl WindowProcedure for Internal {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    log::trace!("{window:?} | {message:?}");
    match message {
      Message::Create(_) | Message::SettingChange(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
      },
      Message::Destroy => {
        window.quit(); // SHOULD BE CHANGED
      },
      _ => (),
    }

    None
  }
}
