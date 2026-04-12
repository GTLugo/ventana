// Linux + Wayland
#[cfg(wayland_platform)]
pub use backend_wayland as wayland;
// Windows
#[cfg(windows_platform)]
pub use backend_win32 as win32;
// Linux + X11
#[cfg(x11_platform)]
pub use backend_x11 as x11;
#[cfg(wayland_platform)]
pub use wayland::Wayland;
#[cfg(windows_platform)]
pub use win32::Win32;
#[cfg(x11_platform)]
pub use x11::X11;
use {
  crate::monitor::Monitor,
  std::{
    collections::VecDeque,
    sync::Arc,
  },
  ventana_hal::{
    backend::Backend as BackendImpl,
    error::RequestError,
    settings::WindowSettings,
    window::BackendWindow,
  },
};

#[derive(Clone)]
pub struct Backend {
  backend: &'static dyn BackendImpl,
}

impl<T: BackendImpl + 'static> From<&'static T> for Backend {
  fn from(backend: &'static T) -> Self {
    Self { backend }
  }
}

impl Backend {
  /// Attempts to select a backend from the first-party backend implementations. Returns `RequestError::NotSupported` if none are available.
  pub fn auto() -> Result<Self, RequestError> {
    #[allow(unreachable_code)]
    {
      #[cfg(windows_platform)]
      return Ok(Win32::instance().into());
      #[cfg(x11_platform)]
      return Ok(X11::instance().into());
      #[cfg(wayland_platform)]
      return Ok(Wayland::instance().into());
      Err(RequestError::NotSupported("No supported backend available to auto-select from."))
    }
  }

  pub(crate) fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    self.backend.create_window(settings)
  }

  pub fn name(&self) -> &'static str {
    self.backend.name()
  }

  pub fn list_available_monitors(&self) -> Result<VecDeque<Monitor>, RequestError> {
    Ok(
      self
        .backend
        .list_available_monitors()?
        .into_iter()
        .map(Monitor::new)
        .collect(),
    )
  }

  pub fn primary_monitor(&self) -> Result<Monitor, RequestError> {
    self.backend.primary_monitor().map(Monitor::new)
  }
}
