#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))] // TODO: Swap this out for a stub impl on other platforms.

mod event;
pub mod monitor;
pub mod window;

use {
  self::{
    monitor::X11Monitor,
    window::X11Window,
  },
  std::{
    collections::VecDeque,
    sync::{
      Arc,
      LazyLock,
    },
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
  }
}

pub struct X11State {
  connection: XCBConnection,
  default_screen_index: usize,
  database: Database,
  atoms: Atoms,
}

#[derive(Clone)]
pub struct X11(Arc<X11State>);

impl X11 {
  pub fn connection() -> &'static XCBConnection {
    &Self::instance().0.connection
  }

  pub fn default_screen_id() -> usize {
    Self::instance().0.default_screen_index
  }

  pub fn default_screen() -> &'static Screen {
    &Self::connection().setup().roots[Self::instance().0.default_screen_index]
  }

  pub fn database() -> &'static Database {
    &Self::instance().0.database
  }

  pub fn atoms() -> &'static Atoms {
    &Self::instance().0.atoms
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

impl Backend for X11 {
  fn instance() -> &'static Self
  where
    Self: Sized,
  {
    static INSTANCE: LazyLock<X11> = LazyLock::new(|| {
      let (connection, default_screen_index) = XCBConnection::connect(None).unwrap();
      let database = new_from_default(&connection).unwrap();
      let atoms = Atoms::new(&connection).unwrap().reply().unwrap();
      X11(Arc::new(X11State {
        connection,
        default_screen_index,
        database,
        atoms,
      }))
    });
    &INSTANCE
  }

  fn name(&self) -> &'static str {
    "X11"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(X11Window::new(settings)?))
  }

  fn list_available_monitors(&self) -> VecDeque<Arc<dyn BackendMonitor>> {
    X11Monitor::list_available()
      .into_iter()
      .map(|m| Arc::new(m) as _)
      .collect()
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
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
}
