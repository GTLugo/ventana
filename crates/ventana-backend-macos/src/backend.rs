#![cfg(target_os = "macos")]

use {
  crate::MacOS,
  ventana_hal::error::RequestError,
};

impl MacOS {
  pub fn new() -> Option<Self> {
    Some(Self)
  }

  pub fn connect() -> Result<(), RequestError> {
    Err(RequestError::Ignored)
  }

  pub fn connection() -> Result<(), RequestError> {
    Err(RequestError::Ignored)
  }
}
