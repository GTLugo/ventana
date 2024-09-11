use ventana_hal::{Backend, Window as HalWindow};
use ventana_types::settings::WindowSettings;

pub struct X11Backend;

impl Backend for X11Backend {
  fn name() -> &'static str {
    "X11"
  }

  fn create_window(&self, settings: WindowSettings) -> Box<dyn HalWindow> {
    Box::new(Window::new(settings))
  }
}

pub struct Window;

impl HalWindow for Window {
  fn new(_settings: WindowSettings) -> Self {
    Self
  }

  fn title(&self) -> String {
    todo!()
  }
}
