mod event;
mod monitor;
mod window;

#[cfg(windows)]
use std::sync::{
  RwLockReadGuard,
  RwLockWriteGuard,
};

use {
  std::sync::RwLock,
  ventana_hal::input::Input,
};
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

#[derive(Debug, Default)]
pub struct Win32 {
  pub(crate) input: RwLock<Input>,
}

#[cfg(windows)]
impl Win32 {
  fn input() -> RwLockReadGuard<'static, Input>
  where
    Self: Sized,
  {
    Self::instance().unwrap().input.read().unwrap()
  }

  fn input_mut() -> RwLockWriteGuard<'static, Input>
  where
    Self: Sized,
  {
    Self::instance().unwrap().input.write().unwrap()
  }
}

#[allow(unused)]
impl Backend for Win32 {
  #[cfg(windows)]
  fn instance() -> Option<&'static Self>
  where
    Self: Sized,
  {
    use std::sync::LazyLock;
    static INSTANCE: LazyLock<Win32> = LazyLock::new(Win32::default);
    Some(&INSTANCE)
  }

  fn is_available() -> bool
  where
    Self: Sized,
  {
    cfg!(windows)
  }

  fn name(&self) -> &'static str {
    "Win32"
  }

  #[cfg(windows)]
  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(self::window::Win32Window::new(settings)?))
  }

  #[cfg(windows)]
  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    Ok(
      win64::user::Monitor::available()
        .into_iter()
        .map(self::monitor::Win32Monitor)
        .map(|m| Arc::new(m) as _)
        .collect(),
    )
  }

  #[cfg(windows)]
  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    Ok(Arc::new(self::monitor::Win32Monitor(win64::user::Monitor::primary())))
  }
}
