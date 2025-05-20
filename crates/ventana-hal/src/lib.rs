pub mod context;
pub mod event;
pub mod input;
pub mod provider;
pub mod settings;
pub mod types;
pub mod window;

pub use keyboard_types as keyboard;
pub use dpi;

use std::error::Error;

use thiserror::Error;

#[derive(Error, Debug)]
pub enum WindowCreationError {
  #[error(
    "no valid default backend selected for ventana. Check feature flags or try manually selecting a compatible backend."
  )]
  NoBackend,
  #[error("{0}")]
  GenericError(Box<dyn Error>),
}

