#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))] // TODO: Swap this out for a stub impl on other platforms.

mod event;
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
  fn instance() -> impl Backend
  where
    Self: Sized,
  {
    Self
  }

  fn name(&self) -> &'static str {
    "X11"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(X11Window::new(settings)?))
  }

  fn primary_monitor(&self) -> Result<Arc<dyn ventana_hal::monitor::BackendMonitor>, RequestError> {
    todo!()
  }
}
