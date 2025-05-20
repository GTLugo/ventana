use keyboard_types::Code;

use crate::{WindowCreationError, settings::WindowSettings, window::BackendWindow};

/// Use this to create the backend-specific window object
pub trait WindowProvider: Send + Sync {
  fn create_window(&self, settings: WindowSettings) -> Result<Box<dyn BackendWindow>, WindowCreationError>;
}

pub trait InputProvider: Send + Sync {
  fn key_to_scancode(&self, key: Code) -> Option<u32>;
  fn scancode_to_key(&self, scancode: u32) -> Code;
}
