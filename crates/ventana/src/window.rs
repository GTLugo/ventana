pub mod iter;

use {
  crate::backend,
  std::sync::Arc,
  ventana_hal::{
    backend::Backend,
    dpi::{
      Position,
      Size,
    },
    error::RequestError,
    event::Event,
    settings::WindowSettings,
    types::Visibility,
    window::{
      BackendWindow,
      WindowId,
    },
  },
};

pub struct Window
where
  Self: Send + Sync,
{
  #[allow(unused)]
  backend: Arc<dyn Backend>,
  window: Arc<dyn BackendWindow>,
}

impl Window {
  pub fn new(options: WindowOptions) -> Result<Self, RequestError> {
    let settings = options.clone().into();
    let Some(backend) = options.backend else {
      return Err(RequestError::NotSupported("No backend selected"));
    };
    let window = backend.create_window(settings)?;
    Ok(Self { backend, window })
  }

  pub fn id(&self) -> WindowId {
    self.window.id()
  }

  pub fn title(&self) -> String {
    self.window.title()
  }

  pub fn size(&self) -> Size {
    self.window.size()
  }

  pub fn position(&self) -> Position {
    self.window.position()
  }

  pub fn next_event(&self) -> Option<Event> {
    self.window.next()
  }
}

#[derive(Clone)]
pub struct WindowOptions {
  pub backend: Option<Arc<dyn Backend>>,
  pub title: &'static str,
  pub size: Size, // Maybe should make this optional and have backend handle None case
  pub position: Option<Position>,
  pub visibility: Visibility,
}

impl Default for WindowOptions {
  fn default() -> Self {
    Self {
      backend: Self::auto_select_backend(),
      title: "Window",
      size: Size::Logical((800.0, 500.0).into()),
      position: None,
      visibility: Default::default(),
    }
  }
}

impl WindowOptions {
  #[allow(unreachable_code)]
  fn auto_select_backend() -> Option<Arc<dyn Backend>> {
    #[cfg(windows_platform)]
    return Some(backend::Win32::instance());
    #[cfg(x11_platform)]
    return Some(backend::Wayland::instance());
    None
  }
}

impl From<WindowOptions> for WindowSettings {
  fn from(options: WindowOptions) -> Self {
    Self {
      title: options.title.into(),
      size: options.size,
      position: options.position,
      visibility: options.visibility,
    }
  }
}
