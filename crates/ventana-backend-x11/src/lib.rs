mod backend;
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
    static INSTANCE: std::sync::LazyLock<Option<X11>> = std::sync::LazyLock::new(X11::new);
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
