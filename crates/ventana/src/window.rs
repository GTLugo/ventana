pub mod iter;
pub mod raw;

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
    types::{
      Flow,
      Visibility,
    },
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

impl std::fmt::Debug for Window {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_struct("Window")
      .field("backend", &self.backend.name())
      .field("window", &self.window.id())
      .finish()
  }
}

impl std::fmt::Display for Window {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{backend}({id})", backend = self.backend.name(), id = self.window.id())
  }
}

impl Window {
  pub fn new(options: WindowOptions) -> Result<Self, RequestError> {
    let settings: WindowSettings = options.clone().into();
    let Some(backend) = options.backend else {
      return Err(RequestError::NotSupported("No backend selected"));
    };

    log::trace!("Creating window `{}`", settings.title);

    let window = backend.create_window(settings)?;
    Ok(Self { backend, window })
  }

  pub fn id(&self) -> WindowId {
    self.window.id()
  }

  pub fn title(&self) -> String {
    self.window.title()
  }

  pub fn inner_size(&self) -> Size {
    self.window.inner_size()
  }

  pub fn outer_size(&self) -> Size {
    self.window.outer_size()
  }

  pub fn inner_position(&self) -> Position {
    self.window.inner_position()
  }

  pub fn outer_position(&self) -> Position {
    self.window.outer_position()
  }

  pub fn next_event(&self) -> Option<Event> {
    self.window.next_event()
  }
}

#[derive(Clone)]
pub struct WindowOptions {
  pub backend: Option<Arc<dyn Backend>>,
  pub title: &'static str,
  pub size: Size, // Maybe should make this optional and have backend handle None case
  pub position: Option<Position>,
  pub visibility: Visibility,
  pub flow: Flow,
  pub close_on_x: bool,
}

impl Default for WindowOptions {
  fn default() -> Self {
    Self {
      backend: Self::auto_select_backend(),
      title: "Window",
      size: Size::Logical((800.0, 500.0).into()),
      position: None,
      visibility: Default::default(),
      flow: Default::default(),
      close_on_x: true,
    }
  }
}

impl WindowOptions {
  #[allow(unreachable_code)]
  fn auto_select_backend() -> Option<Arc<dyn Backend>> {
    #[cfg(windows_platform)]
    return Some(backend::Win32::instance());
    #[cfg(x11_platform)]
    return Some(backend::X11::instance());
    #[cfg(wayland_platform)]
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
      flow: options.flow,
      close_on_x: options.close_on_x,
    }
  }
}
