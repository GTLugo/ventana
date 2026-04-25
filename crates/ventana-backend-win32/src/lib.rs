mod event;
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Win32;

#[allow(unused)]
impl Backend for Win32 {
  #[cfg(target_os = "windows")]
  fn new() -> Option<&'static Self>
  where
    Self: Sized,
  {
    use std::sync::LazyLock;
    static INSTANCE: LazyLock<Win32> = LazyLock::new(|| Win32);
    Some(&INSTANCE)
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

  #[cfg(target_os = "windows")]
  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(self::window::Win32Window::new(settings)?))
  }

  #[cfg(target_os = "windows")]
  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    Ok(
      win64::user::Monitor::available()
        .into_iter()
        .map(self::monitor::Win32Monitor)
        .map(|m| Arc::new(m) as _)
        .collect(),
    )
  }

  #[cfg(target_os = "windows")]
  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    Ok(Arc::new(self::monitor::Win32Monitor(win64::user::Monitor::primary())))
  }
}
