#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))] // TODO: Swap this out for a stub impl on other platforms.

use {
  std::sync::Arc,
  ventana_hal::{
    backend::Backend,
    error::RequestError,
    settings::WindowSettings,
    window::BackendWindow,
  },
};

pub struct Wayland;

impl Backend for Wayland {
  fn instance() -> impl Backend
  where
    Self: Sized,
  {
    Self
  }

  fn name(&self) -> &'static str {
    "Wayland"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    todo!()
  }
}
