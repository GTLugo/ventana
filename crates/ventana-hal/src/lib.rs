pub mod context;
pub mod input;
pub mod message;
pub mod position;
pub mod provider;
pub mod settings;
pub mod size;
pub mod types;
pub mod window;

use std::error::Error;

use thiserror::Error;
pub use window_input::*;

#[derive(Error, Debug)]
pub enum WindowCreationError {
  #[error("no valid default backend selected for ventana. Check feature flags or try manually selecting a compatible backend.")]
  NoBackend,
  #[error("{0}")]
  GenericError(Box<dyn Error>),
}