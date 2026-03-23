use {
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
  win64::prelude as win64,
};

pub struct Win32Window {
  hwnd: win64::Window,
  settings: Arc<RwLock<WindowSettings>>,
}

impl Win32Window {
  #[allow(clippy::new_ret_no_self)]
  pub fn new(settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    let class = win64::WindowClass::builder()
      .name("Window Class")
      .register()
      .map_to_os_err()?;
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
      settings: Arc::new(RwLock::new(settings)),
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
    self.settings.read().unwrap().title.to_string()
  }

  fn size(&self) -> Size {
    self.settings.read().unwrap().size
  }

  fn position(&self) -> Position {
    self.settings.read().unwrap().position.unwrap() // TODO: Handle None case (probably change this entirely)
  }

  fn key(&self, keycode: Code) -> KeyState {
    todo!()
  }

  fn mouse(&self, button: MouseButton) -> KeyState {
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

impl win64::WindowProcedure for Internal {
  fn on_message(&mut self, window: &win64::Window, message: &win64::Message) -> Option<win64::LResult> {
    log::trace!("{window:?} | {message:?}");

    None
  }
}
