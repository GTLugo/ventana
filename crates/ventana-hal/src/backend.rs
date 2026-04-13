use {
  crate::{
    error::RequestError,
    monitor::BackendMonitor,
    settings::WindowSettings,
    window::BackendWindow,
  },
  std::{
    collections::VecDeque,
    sync::Arc,
  },
};

pub trait Backend: Send + Sync {
  /// This should return a reference to a static instance of the backend, as opposed to creating a new instance every time.
  /// This is because some backends may need to maintain global state.
  /// 
  /// This should return `None` if the backend is not available on the current platform.
  fn instance() -> Option<&'static Self>
  where
    Self: Sized;

  /// This should be a quick check to see if the backend is available on the current platform. One should implement this with
  /// compile-time cfg checks and env var checks as necessary. Then, use cfgs to avoid compiling the methods of the backend if
  /// it is not available.
  ///
  /// For now this is unused, but in the future, this will be used to determine which backend to use when `Backend::auto()`
  /// is called.
  fn is_available() -> bool
  where
    Self: Sized;

  /// The name of the backend, used for logging and debugging purposes.
  fn name(&self) -> &'static str;

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    let _ = settings;
    Err(RequestError::NotSupported("Backend is not available to create a window"))
  }

  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    Err(RequestError::NotSupported("Backend is not available to list available monitors"))
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    Err(RequestError::NotSupported("Backend is not available to get the primary monitor"))
  }
}
