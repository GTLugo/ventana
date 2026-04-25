pub mod backend;

use {
  backend::{
    linux::Linux,
    macos::MacOS,
    win32::Win32,
  },
  hal::{
    backend::Backend,
    error::RequestError,
    monitor::BackendMonitor,
    settings::WindowSettings,
    window::BackendWindow,
  },
  std::{
    collections::VecDeque,
    fmt::Debug,
    sync::{
      Arc,
      LazyLock,
    },
  },
};

// TODO: Add a way to specify a preferred backend selection order
#[derive(Clone)]
pub struct AutoBackend(&'static dyn Backend);

impl Debug for AutoBackend {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl Backend for AutoBackend {
  fn new() -> Option<&'static Self>
  where
    Self: Sized,
  {
    static INSTANCE: LazyLock<Option<AutoBackend>> = LazyLock::new(|| AutoBackend::auto().ok().map(AutoBackend));
    INSTANCE.as_ref()
  }

  fn is_available() -> bool
  where
    Self: Sized,
  {
    Linux::is_available()
  }

  fn name(&self) -> &'static str {
    self.0.name()
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    self.0.create_window(settings)
  }

  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    self.0.list_available_monitors()
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    self.0.primary_monitor()
  }
}

impl AutoBackend {
  /// Attempts to select a backend from the first-party backend implementations. Returns `RequestError::NotSupported` if none are available.
  fn auto() -> Result<&'static dyn Backend, RequestError> {
    Win32::instance()
      .or_else(MacOS::instance)
      .or_else(Linux::instance)
      .ok_or(RequestError::not_supported("No supported backend available to auto-select from."))
  }
}
