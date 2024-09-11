use ventana_hal::{Backend, Window as HalWindow};
use ventana_types::settings::WindowSettings;

pub struct Win32Backend;

impl Backend for Win32Backend {
  fn name() -> &'static str {
    "Win32"
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
    println!("New Win32 window!");
    Self { settings }
  }

  fn title(&self) -> String {
    self.settings.title.clone()
  }
}
