mod backend;
mod keyboard;
mod monitor;
mod window;

#[allow(unused)]
use {
  std::{
    collections::VecDeque,
    sync::Arc,
  },
  ventana_hal::{
    backend::Backend,
    error::RequestError,
    monitor::BackendMonitor,
    settings::WindowSettings,
    window::BackendWindow,
  },
};

#[derive(Clone)]
pub struct X11(#[cfg(linux_platform)] Arc<crate::backend::X11State>);

impl Backend for X11 {
  #[cfg(linux_platform)]
  fn instance() -> Option<&'static Self>
  where
    Self: Sized,
  {
    static INSTANCE: std::sync::LazyLock<Option<X11>> = std::sync::LazyLock::new(|| {
      use {
        self::backend::{
          Atoms,
          X11State,
        },
        x11rb::{
          resource_manager::new_from_default,
          xcb_ffi::XCBConnection,
        },
      };

      let (connection, default_screen_index) = match XCBConnection::connect(None) {
        Ok(connection) => connection,
        Err(error) => {
          log::error!("Failed to connect to X server: `{error}`");
          return None;
        },
      };
      let database = new_from_default(&connection).unwrap();
      let atoms = Atoms::new(&connection).unwrap().reply().unwrap();
      Some(X11(Arc::new(X11State { connection, default_screen_index, database, atoms })))
    });
    INSTANCE.as_ref()
  }

  #[cfg(linux_platform)]
  fn is_available() -> bool
  where
    Self: Sized,
  {
    backend::is_available()
  }

  fn name(&self) -> &'static str {
    "X11"
  }

  #[cfg(linux_platform)]
  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    backend::create_window(settings)
  }

  #[cfg(linux_platform)]
  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    backend::list_available_monitors()
  }

  #[cfg(linux_platform)]
  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    backend::primary_monitor()
  }
}
