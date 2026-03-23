pub mod window;

use {
  self::window::Win32Window,
  std::sync::Arc,
  ventana_hal::{
    backend::Backend,
    error::RequestError,
    keyboard::Code,
    settings::WindowSettings,
    window::BackendWindow,
  },
};

pub struct Win32;

#[allow(unused)]
impl Backend for Win32 {
  fn instance() -> Arc<dyn Backend>
  where
    Self: Sized,
  {
    Arc::new(Self)
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Win32Window::new(settings)
  }

  fn key_to_scancode(&self, key: Code) -> Option<u32> {
    todo!()
  }

  fn scancode_to_key(&self, scancode: u32) -> Code {
    todo!()
  }
}
