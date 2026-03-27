#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))] // TODO: Swap this out for a stub impl on other platforms.

pub mod window;

use {
  self::window::X11Window,
  std::sync::Arc,
  ventana_hal::{
    backend::Backend,
    error::RequestError,
    settings::WindowSettings,
    window::BackendWindow,
  },
};

pub struct X11;

impl Backend for X11 {
  fn instance() -> Arc<dyn Backend>
  where
    Self: Sized,
  {
    Arc::new(Self)
  }

  fn name(&self) -> &'static str {
    "X11"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    X11Window::new()
  }
}

// impl WindowProvider for WaylandBackend {
//   fn create_window(&self, settings: WindowSettings) -> Box<dyn HalWindow> {
//     Box::new(Window { settings })
//   }
// }

// pub struct Window {
//   settings: WindowSettings,
// }

// impl HalWindow for Window {
//   fn next(&self) -> Option<Message> {
//     None
//   }

//   fn title(&self) -> String {
//     self.settings.title.clone()
//   }

//   fn size(&self) -> Size {
//     self.settings.size
//   }

//   fn position(&self) -> Position {
//     self.settings.position
//   }
// }
