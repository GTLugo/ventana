#![cfg(target_os = "windows")] // TODO: Swap this out for a stub impl on other platforms.

mod event;
pub mod window;

use {
  self::window::Win32Window,
  std::sync::Arc,
  ventana_hal::{
    backend::Backend,
    error::RequestError,
    settings::WindowSettings,
    window::BackendWindow,
  },
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Win32;

#[allow(unused)]
impl Backend for Win32 {
  fn instance() -> impl Backend
  where
    Self: Sized,
  {
    Self
  }

  fn name(&self) -> &'static str {
    "Win32"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Win32Window::new(settings)
  }
}
