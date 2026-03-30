#![cfg(target_os = "windows")] // TODO: Swap this out for a stub impl on other platforms.

mod event;
mod monitor;
pub mod window;

use {
  self::{
    monitor::Win32Monitor,
    window::Win32Window,
  },
  std::sync::Arc,
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
  fn new() -> impl Backend
  where
    Self: Sized,
  {
    Self
  }

  fn name(&self) -> &'static str {
    "Win32"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(Win32Window::new(settings)?))
  }

  fn list_available_monitors(&self) -> VecDeque<Arc<dyn BackendMonitor>> {
    Win32Monitor::list_available()
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    Ok(Arc::new(Win32Monitor::primary()))
  }
}
