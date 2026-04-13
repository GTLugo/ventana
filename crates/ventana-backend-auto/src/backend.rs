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
      let windows = crate::Win32::is_available();
      #[cfg(x11_platform)]
      return windows || crate::X11::is_available();
      #[cfg(wayland_platform)]
      return windows || crate::Wayland::is_available();
      windows
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
    #[allow(clippy::unnecessary_lazy_evaluations)]
    crate::Win32::instance()
      .map(|b| b as _)
      .or_else(|| {
        #[cfg(x11_platform)]
        return crate::X11::instance().map(|b| b as _);
        #[cfg(wayland_platform)]
        return crate::Wayland::instance().map(|b| b as _);
        None
      })
      .ok_or(RequestError::NotSupported("No supported backend available to auto-select from."))
  }
}
