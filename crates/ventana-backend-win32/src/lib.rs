#![cfg(target_os = "windows")] // TODO: Swap this out for a stub impl on other platforms.

mod event;
mod monitor;
pub mod window;

use {
  self::{
    monitor::Win32Monitor,
    window::Win32Window,
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
    settings::WindowSettings,
    window::BackendWindow,
  },
  win64::user::Monitor,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Win32;

#[allow(unused)]
impl Backend for Win32 {
  fn instance() -> &'static Self
  where
    Self: Sized,
  {
    static INSTANCE: LazyLock<Win32> = LazyLock::new(|| Win32);
    &INSTANCE
  }

  fn name(&self) -> &'static str {
    "Win32"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(Win32Window::new(settings)?))
  }

  fn list_available_monitors(&self) -> VecDeque<Arc<dyn BackendMonitor>> {
    Monitor::available()
      .into_iter()
      .map(Win32Monitor)
      .map(|m| Arc::new(m) as _)
      .collect()
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    Ok(Arc::new(Win32Monitor(Monitor::primary())))
  }
}
