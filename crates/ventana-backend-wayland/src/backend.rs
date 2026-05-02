#![cfg(linux_platform)]

use {
  crate::Wayland,
  std::sync::Arc,
  ventana_hal::backend::Backend,
  wayland_client::Connection,
};

pub struct WaylandState {
  connection: Connection,
}

impl Wayland {
  pub fn new() -> Option<Self> {
    let connection = match Connection::connect_to_env() {
      Ok(connection) => connection,
      Err(error) => {
        log::error!("failed to connect to wayland server: `{error}`");
        return None;
      },
    };
    Some(Self(Arc::new(WaylandState { connection })))
  }

  pub fn connection() -> &'static Connection {
    &Self::instance().unwrap().0.connection
  }
}
