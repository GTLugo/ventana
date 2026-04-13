use {
  hal::{
    backend::Backend,
    error::RequestError,
    monitor::BackendMonitor,
    settings::WindowSettings,
    window::BackendWindow,
  },
  std::{
    collections::VecDeque,
    sync::{
      Arc,
      LazyLock,
    },
  },
};

#[derive(Clone)]
pub struct AutoBackend(&'static dyn Backend);

impl Backend for AutoBackend {
  fn instance() -> Option<&'static Self>
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
    #[allow(unreachable_code)]
    {
      #[cfg(windows_platform)]
      return Win32::is_available();
      #[cfg(x11_platform)]
      return crate::X11::is_available();
      #[cfg(wayland_platform)]
      return crate::Wayland::is_available();
      false
    }
  }

  fn name(&self) -> &'static str {
    "Auto"
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
    let err = RequestError::NotSupported("No supported backend available to auto-select from.");
    #[allow(unreachable_code)]
    {
      #[cfg(windows_platform)]
      return crate::Win32::instance().ok_or(err).map(|b| b as _);
      #[cfg(x11_platform)]
      return crate::X11::instance().ok_or(err).map(|b| b as _);
      #[cfg(wayland_platform)]
      return crate::Wayland::instance().ok_or(err).map(|b| b as _);
      Err(err)
    }
  }
}
