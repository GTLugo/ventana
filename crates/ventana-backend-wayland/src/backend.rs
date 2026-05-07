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
    globals::{
      GlobalList,
      registry_queue_init,
    },
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

    let (globals, event_queue) = match registry_queue_init(&connection) {
      Ok((globals, event_queue)) => (globals, Mutex::new(event_queue)),
      Err(error) => {
        log::error!("failed to connect to wayland server: `{error}`");
        return None;
      },
    };

    Some(Self(Arc::new(WaylandConnection { connection, display, event_queue, globals })))
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

  pub fn globals() -> &'static GlobalList {
    &Self::instance().unwrap().0.globals
  }
}

pub struct WaylandConnection {
  connection: Connection,
  display: WlDisplay,
  event_queue: Mutex<EventQueue<WindowState>>,
  globals: GlobalList,
}
