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
    error::RequestError,
    monitor::BackendMonitor,
    os_error,
    settings::WindowSettings,
    window::BackendWindow,
  },
  x11rb::{
    atom_manager,
    connection::Connection,
    protocol::{
      randr::ConnectionExt,
      xproto::Screen,
    },
    resource_manager::{
      Database,
      new_from_default,
    },
    rust_connection::RustConnection,
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
  connection: RustConnection,
  default_screen_index: usize,
  database: Database,
  atoms: Atoms,
}

#[derive(Clone)]
pub struct X11(Arc<X11State>);

impl X11 {
  // fn state_lock(&self) -> MutexGuard<'_, X11State> {
  //   self.0.lock().unwrap()
  // }

  pub fn get_xft_dpi(&self) -> f64 {
    self
      .0
      .database
      .get_value::<f64>("Xft.dpi", "")
      .ok()
      .flatten()
      .or_else(|| self.0.database.get_value::<f64>("Xft/DPI", "").ok().flatten())
      .map(|dpi| if dpi > 0.0 { dpi } else { 96.0 })
      .unwrap_or(96.0)
  }

  pub fn connection(&self) -> &RustConnection {
    &self.0.connection
  }

  pub fn default_screen(&self) -> &Screen {
    self
      .0
      .connection
      .setup()
      .roots
      .get(self.0.default_screen_index)
      .unwrap()
  }

  pub fn atoms(&self) -> &Atoms {
    &self.0.atoms
  }
}

impl Backend for X11 {
  fn instance() -> &'static Self
  where
    Self: Sized,
  {
    static INSTANCE: LazyLock<X11> = LazyLock::new(|| {
      let (connection, default_screen_index) = x11rb::connect(None).unwrap();
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
    let x11 = X11::instance();
    let screen = x11.default_screen();
    let scale_factor = x11.get_xft_dpi() / X11Monitor::DEFAULT_DPI;
    x11
      .connection()
      .randr_get_monitors(screen.root, true)
      .unwrap()
      .reply()
      .unwrap()
      .monitors
      .into_iter()
      .map(|info| {
        Arc::new(X11Monitor {
          id: info.name,
          scale_factor,
          primary: info.primary,
          automatic: info.automatic,
          x: info.x,
          y: info.y,
          width: info.width,
          height: info.height,
          width_in_millimeters: info.width_in_millimeters,
          height_in_millimeters: info.height_in_millimeters,
        }) as _
      })
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
