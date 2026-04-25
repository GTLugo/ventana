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

///
/// The primary backend trait.
///
/// This interface allows backends to be implemented for multiple platforms. Each backend should implement every function,
/// but default implementations are provided that return `RequestError::NotSupported` so that cfg attributes can be used
/// to avoid compiling code for unsupported platforms.
///
pub trait Backend: Send + Sync {
  /// This should return a reference to a static instance of the backend, as opposed to creating a new instance every time.
  /// This is because some backends may need to maintain global state.
  /// This should also return `None` if the backend is not implemented on the current platform.
  fn instance() -> Option<&'static Self>
  where
    Self: Sized + 'static,
  {
    None
  }

  /// Returns a reference to the instance as `dyn Backend`
  fn backend() -> Option<&'static dyn Backend>
  where
    Self: Sized + 'static,
  {
    Self::instance().map(|b| b as _)
  }

  /// This should be a quick check to see if the backend is available on the current platform. One should implement this with
  /// compile-time cfg checks and env var checks as necessary. Then, use cfgs to avoid compiling the methods of the backend if
  /// it is not available.
  ///
  /// For now this is unused, but in the future, this will be used to determine which backend to use when `Backend::auto()`
  /// is called.
  fn is_available() -> bool
  where
    Self: Sized,
  {
    false
  }

  /// The name of the backend, used for logging and debugging purposes.
  fn name(&self) -> &'static str;

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    let _ = settings;
    Err(RequestError::not_supported(format!(
      "`{}` backend is not available to create a window",
      self.name()
    )))
  }

  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    Err(RequestError::not_supported(format!(
      "`{}` backend is not available to list available monitors",
      self.name()
    )))
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    Err(RequestError::not_supported(format!(
      "`{}` backend is not available to get the primary monitor",
      self.name()
    )))
  }
}
