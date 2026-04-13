#![cfg(target_os = "windows")]

use {
  crate::{
    monitor::Win32Monitor,
    window::Win32Window,
  },
  std::{
    collections::VecDeque,
    sync::Arc,
  },
  ventana_hal::{
    error::RequestError,
    monitor::BackendMonitor,
    settings::WindowSettings,
    window::BackendWindow,
  },
  win64::user::Monitor,
};

pub(crate) fn create_window(settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
  Ok(Arc::new(Win32Window::new(settings)?))
}

pub(crate) fn list_available_monitors() -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
  Ok(
    Monitor::available()
      .into_iter()
      .map(Win32Monitor)
      .map(|m| Arc::new(m) as _)
      .collect(),
  )
}

pub(crate) fn primary_monitor() -> Result<Arc<dyn BackendMonitor>, RequestError> {
  Ok(Arc::new(Win32Monitor(Monitor::primary())))
}
