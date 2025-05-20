use std::sync::Arc;
use ventana_hal::{
  context::Backend, dpi::{Position, Size}, event::Event, settings::WindowSettings, window::{BackendWindow, WindowId}, WindowCreationError
};
use crate::backend;

pub struct Window
where
  Self: Send + Sync,
{
  backend: Arc<dyn Backend>,
  window: Box<dyn BackendWindow>,
}

impl Window {
  pub fn builder() -> WindowBuilder {
    WindowBuilder::new()
  }

  fn new(backend: Arc<dyn Backend>, settings: &WindowSettings) -> Result<Self, WindowCreationError> {
    let window = backend.create_window(settings.clone())?;
    Ok(Self { backend, window })
  }

  pub fn id(&self) -> WindowId {
    self.window.id()
  }

  pub fn title(&self) -> String {
    self.window.title(&*self.backend)
  }

  pub fn size(&self) -> Size {
    self.window.size(&*self.backend)
  }

  pub fn position(&self) -> Position {
    self.window.position(&*self.backend)
  }

  pub fn next_event(&self) -> Option<Event> {
    self.window.next(&*self.backend)
  }
}

pub struct WindowBuilder {
  backend: Option<Arc<dyn Backend>>,
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
    fn pick() -> Option<Arc<dyn Backend>> {
      #[cfg(windows_platform)]
      return Some(Arc::new(backend::Win32));
      #[cfg(x11_platform)]
      return Some(Arc::new(backend::Wayland));
      None
    }

    Self {
      backend: pick(),
      settings: WindowSettings::default(),
    }
  }

  pub fn with_backend(&mut self, backend: impl Backend) -> &mut Self {
    self.backend = Some(Arc::new(backend));
    self
  }

  pub fn with_settings(&mut self, settings: WindowSettings) -> &mut Self {
    self.settings = settings;
    self
  }

  pub fn build(&self) -> Result<Window, WindowCreationError> {
    let Some(backend) = &self.backend else {
      return Err(WindowCreationError::NoBackend);
    };
    Window::new(backend.clone(), &self.settings)
  }
}
