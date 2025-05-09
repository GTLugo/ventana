use window_input::keyboard::PhysicalKey;

use crate::{settings::WindowSettings, window::Window, WindowCreationError};

/// Use this to create the backend-specific window object
pub trait WindowProvider: Send + Sync {
  fn create_window(&self, settings: WindowSettings) -> Result<Box<dyn Window>, WindowCreationError>;
}

pub trait InputProvider: Send + Sync {
  fn key_to_scancode(&self, key: PhysicalKey) -> Option<u32>;
  fn scancode_to_key(&self, scancode: u32) -> PhysicalKey;
}
