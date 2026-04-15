#![cfg(linux_platform)]

use {
  crate::Wayland,
  std::sync::Arc,
  ventana_hal::error::RequestError,
};

pub struct WaylandState {
  connection: (),
}

impl Wayland {
  pub fn new() -> Option<Self> {
    let connection = match Self::connect() {
      Ok(connection) => connection,
      Err(error) => {
        log::error!("Failed to connect to Wayland server: `{error}`");
        return None;
      },
    };
    Some(Self(Arc::new(WaylandState { connection })))
  }

  pub fn connect() -> Result<(), RequestError> {
    Err(RequestError::Ignored)
  }

  pub fn connection() -> Result<(), RequestError> {
    Err(RequestError::Ignored)
  }
}
