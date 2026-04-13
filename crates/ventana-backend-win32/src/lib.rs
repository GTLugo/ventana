mod backend;
mod event;
mod monitor;
mod window;

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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Win32;

#[allow(unused)]
impl Backend for Win32 {
  fn instance() -> Option<&'static Self>
  where
    Self: Sized,
  {
    #[cfg(target_os = "windows")]
    {
      static INSTANCE: std::sync::LazyLock<Win32> = std::sync::LazyLock::new(|| Win32);
      Some(&INSTANCE)
    }
    #[cfg(not(target_os = "windows"))]
    None
  }

  fn is_available() -> bool
  where
    Self: Sized,
  {
    cfg!(target_os = "windows")
  }

  fn name(&self) -> &'static str {
    "Win32"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    #[cfg(target_os = "windows")]
    {
      backend::create_window(settings)
    }
    #[cfg(not(target_os = "windows"))]
    Err(RequestError::NotSupported("Win32 backend is only supported on Windows"))
  }

  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    #[cfg(target_os = "windows")]
    {
      backend::list_available_monitors()
    }
    #[cfg(not(target_os = "windows"))]
    Err(RequestError::NotSupported("Win32 backend is only supported on Windows"))
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    #[cfg(target_os = "windows")]
    {
      backend::primary_monitor()
    }
    #[cfg(not(target_os = "windows"))]
    Err(RequestError::NotSupported("Win32 backend is only supported on Windows"))
  }
}
