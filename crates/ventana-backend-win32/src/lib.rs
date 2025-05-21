pub mod window;

use ventana_hal::{
  WindowCreationError,
  context::Backend,
  keyboard::Code,
  provider::{InputProvider, WindowProvider},
  settings::WindowSettings,
  window::BackendWindow,
};

use self::window::Window;

pub struct Win32;

impl Backend for Win32 {}

impl WindowProvider for Win32 {
  fn create_window(&self, settings: WindowSettings) -> Result<Box<dyn BackendWindow>, WindowCreationError> {
    match Window::new(settings) {
      Ok(window) => Ok(Box::new(window)),
      Err(error) => Err(WindowCreationError::GenericError(Box::new(error))),
    }
  }
}

impl InputProvider for Win32 {
  fn key_to_scancode(&self, key: Code) -> Option<u32> {
    todo!()
  }

  fn scancode_to_key(&self, scancode: u32) -> Code {
    todo!()
  }
}
