#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))]

use {
  crate::{
    X11,
    monitor::X11Monitor,
    window::X11Window,
  },
  std::{
    collections::VecDeque,
    sync::Arc,
  },
  ventana_hal::{
    backend::Backend,
    error::{
      MapToOSError,
      RequestError,
    },
    monitor::BackendMonitor,
    os_error,
    settings::WindowSettings,
    window::BackendWindow,
  },
  x11rb::{
    atom_manager,
    connection::Connection,
    protocol::xproto::{
      ConnectionExt,
      GetGeometryReply,
      Screen,
    },
    resource_manager::{
      Database,
      new_from_default,
    },
    xcb_ffi::XCBConnection,
  },
};

atom_manager! {
  pub Atoms: AtomsCookie {
    WM_PROTOCOLS,
    WM_DELETE_WINDOW,
    _NET_WM_NAME,
    UTF8_STRING,
    VENTANA_REQUEST_REDRAW,
  }
}

pub fn is_available() -> bool {
  XCBConnection::connect(None).is_ok()
}

pub fn create_window(settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
  Ok(Arc::new(X11Window::new(settings)?))
}

pub fn list_available_monitors() -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
  Ok(
    X11Monitor::list_available()
      .into_iter()
      .map(|m| Arc::new(m) as _)
      .collect(),
  )
}

pub fn primary_monitor() -> Result<Arc<dyn BackendMonitor>, RequestError> {
  let monitors = X11Monitor::list_available();
  log::debug!("Available monitors: {monitors:?}");
  let primary = monitors.iter().find(|m| m.primary);
  log::debug!("Primary: {primary:?}");

  Ok(Arc::new(
    primary
      .or_else(|| monitors.front())
      .ok_or_else(|| os_error!("No available monitors to select from"))?
      .clone(),
  ))
}

impl X11 {
  pub(crate) fn new() -> Option<Self> {
    let (connection, default_screen_index) = match XCBConnection::connect(None) {
        Ok(connection) => connection,
        Err(error) => {
          log::error!("Failed to connect to X server: {error}");
          return None;
        },
    };
    let database = new_from_default(&connection).unwrap();
    let atoms = Atoms::new(&connection).unwrap().reply().unwrap();
    Some(Self(Arc::new(X11State {
      connection,
      default_screen_index,
      database,
      atoms,
    })))
  }

  pub fn connection() -> &'static XCBConnection {
    &Self::instance().unwrap().0.connection
  }

  pub fn default_screen_id() -> usize {
    Self::instance().unwrap().0.default_screen_index
  }

  pub fn default_screen() -> &'static Screen {
    &Self::connection().setup().roots[Self::instance().unwrap().0.default_screen_index]
  }

  pub fn database() -> &'static Database {
    &Self::instance().unwrap().0.database
  }

  pub fn atoms() -> &'static Atoms {
    &Self::instance().unwrap().0.atoms
  }

  pub fn geometry() -> Result<GetGeometryReply, RequestError> {
    Ok(
      Self::connection()
        .get_geometry(Self::default_screen().root)
        .map_to_os_err()?
        .reply()
        .map_to_os_err()?,
    )
  }
}

pub struct X11State {
  connection: XCBConnection,
  default_screen_index: usize,
  database: Database,
  atoms: Atoms,
}
