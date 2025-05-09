use ventana_hal::{
  WindowCreationError,
  context::{Backend as HalBackend, Context},
  message::Message,
  position::Position,
  settings::WindowSettings,
  size::Size,
  window::Window as HalWindow,
};

use crate::backend;

pub struct Window
where
  Self: Send + Sync,
{
  context: Context,
  window: Box<dyn HalWindow>,
}

impl Window {
  pub fn builder() -> WindowBuilder {
    WindowBuilder::new()
  }

  fn new(context: Context, settings: &WindowSettings) -> Result<Self, WindowCreationError> {
    let window = context.create_window(settings.clone())?;
    Ok(Self { context, window })
  }

  pub fn title(&self) -> String {
    self.window.title(&self.context)
  }

  pub fn size(&self) -> Size {
    self.window.size(&self.context)
  }

  pub fn position(&self) -> Position {
    self.window.position(&self.context)
  }

  pub fn next_message(&self) -> Option<Message> {
    self.window.next(&self.context)
  }
}

pub struct WindowBuilder {
  backend: Option<Context>,
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
    fn pick() -> Option<Context> {
      #[cfg(windows_platform)]
      return Some(Context::from(backend::win32::Win32));
      #[cfg(x11_platform)]
      return Some(Context::from(backend::x11::X11Backend));
      None
    }

    Self {
      backend: pick(),
      settings: WindowSettings::default(),
    }
  }

  pub fn with_backend(&mut self, backend: impl HalBackend) -> &mut Self {
    self.backend = Some(backend.into());
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
