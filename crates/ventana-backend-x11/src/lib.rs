use ventana_hal::{position::Position, settings::WindowSettings, size::Size, Backend, Window as HalWindow};

pub struct X11Backend;

impl Backend for X11Backend {
  fn name() -> &'static str {
    "X11"
  }

  fn create_window(&self, settings: WindowSettings) -> Box<dyn HalWindow> {
    Box::new(Window::new(settings))
  }
}

pub struct Window {
  settings: WindowSettings,
}

impl HalWindow for Window {
  fn new(settings: WindowSettings) -> Self {
    Self { settings }
  }

  fn title(&self) -> String {
    self.settings.title.clone()
  }

  fn size(&self) -> Size {
    self.settings.size
  }

  fn position(&self) -> Position {
    self.settings.position
  }
}
