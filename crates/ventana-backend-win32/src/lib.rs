use ventana_hal::{position::Position, settings::WindowSettings, size::Size, Backend, Window as HalWindow};

// #[derive(Backend)]
// #[window(Window)]
pub struct Win32;

impl Backend for Win32 {
  fn create_window(&self, settings: WindowSettings) -> Box<dyn HalWindow> {
    Box::new(Window { settings })
  }
}

pub struct Window {
  settings: WindowSettings,
}

impl HalWindow for Window {
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
