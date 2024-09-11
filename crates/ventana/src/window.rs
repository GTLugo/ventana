use ventana_hal::{position::Position, settings::WindowSettings, size::Size, Backend as HalBackend, Window as HalWindow};

use crate::backend;

pub struct Window(Box<dyn HalWindow>);

impl Window {
  pub fn builder() -> WindowBuilder {
    WindowBuilder::new()
  }

  pub fn with_backend(backend: &Backend, settings: &WindowSettings) -> Self {
    let backend = (backend.0)();
    Self(backend.create_window(settings.clone()))
  }

  pub fn new(settings: &WindowSettings) -> Self {
    Self::with_backend(&Backend::default(), settings)
  }

  pub fn title(&self) -> String {
    self.0.title()
  }

  pub fn size(&self) -> Size {
    self.0.size()
  }

  pub fn position(&self) -> Position {
    self.0.position()
  }
}

pub struct Backend(Box<dyn Fn() -> Box<dyn HalBackend> + 'static>);

impl Default for Backend {
  fn default() -> Self {
    #[allow(unreachable_code)]
    Self(Box::new(|| {
      #[cfg(windows_platform)]
      return Box::new(backend::win32::Win32Backend);
      #[cfg(x11_platform)]
      return Box::new(backend::x11::X11Backend);
      panic!("No valid default backend selected for ventana. Check feature flags or try manually selecting a compatible backend.");
    }))
  }
}

impl Backend {
  pub fn select(selector: impl Fn() -> Box<dyn HalBackend> + 'static) -> Self {
    Self(Box::new(selector))
  }
}

pub struct WindowBuilder {
  backend: Backend,
  settings: WindowSettings,
}

impl Default for WindowBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl WindowBuilder {
  pub fn new() -> Self {
    #[allow(unreachable_code)]
    Self {
      backend: Backend::default(),
      settings: WindowSettings::default(),
    }
  }

  pub fn with_backend(&mut self, backend: Backend) -> &mut Self {
    self.backend = backend;
    self
  }

  pub fn build(&self) -> Window {
    Window::with_backend(&self.backend, &self.settings)
  }
}
