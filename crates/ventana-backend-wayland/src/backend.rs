#![cfg(linux_platform)]

use {
  crate::{
    Wayland,
    window::state::WindowState,
  },
  std::sync::{
    Arc,
    Mutex,
    MutexGuard,
  },
  ventana_hal::backend::Backend,
  wayland_client::{
    Connection,
    EventQueue,
    protocol::wl_display::WlDisplay,
  },
};

impl Wayland {
  pub fn new() -> Option<Self> {
    let connection = match Connection::connect_to_env() {
      Ok(connection) => connection,
      Err(error) => {
        log::error!("failed to connect to wayland server: `{error}`");
        return None;
      },
    };
    let display = connection.display();
    let event_queue = Mutex::new(connection.new_event_queue());
    Some(Self(Arc::new(WaylandState { connection, display, event_queue })))
  }

  pub fn connection() -> &'static Connection {
    &Self::instance().unwrap().0.connection
  }

  pub fn display() -> &'static WlDisplay {
    &Self::instance().unwrap().0.display
  }

  pub fn event_queue() -> MutexGuard<'static, EventQueue<WindowState>> {
    Self::instance().unwrap().0.event_queue.lock().unwrap()
  }
}

pub struct WaylandState {
  connection: Connection,
  display: WlDisplay,
  event_queue: Mutex<EventQueue<WindowState>>,
}
