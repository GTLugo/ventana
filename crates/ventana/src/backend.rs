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
  backend: Arc<dyn BackendImpl>,
}

impl<T: BackendImpl + 'static> From<T> for Backend {
  fn from(backend: T) -> Self {
    Self {
      backend: Arc::new(backend),
    }
  }
}

impl Backend {
  pub fn new<T: BackendImpl + 'static>() -> Self {
    Self {
      backend: Arc::new(T::new()),
    }
  }

  // fn instance() -> &'static dyn BackendImpl {
  //   static BACKEND: LazyLock<Backend> = LazyLock::new(|| Backend::auto());
  //   BACKEND.backend.as_ref()
  // }

  /// Attempts to select a backend from the first-party backend implementations. Returns `None` if none are available.
  pub fn auto() -> Option<Self> {
    #[allow(unreachable_code)]
    {
      #[cfg(windows_platform)]
      return Some(Win32.into());
      #[cfg(x11_platform)]
      return Some(X11.into());
      #[cfg(wayland_platform)]
      return Some(Wayland.into());
      None
    }
  }

  pub(crate) fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    self.backend.create_window(settings)
  }

  pub fn name(&self) -> &'static str {
    self.backend.name()
  }

  pub fn list_available_monitors(&self) -> VecDeque<Monitor> {
    self
      .backend
      .list_available_monitors()
      .into_iter()
      .map(Monitor::new)
      .collect()
  }

  pub fn primary_monitor(&self) -> Result<Monitor, RequestError> {
    self.backend.primary_monitor().map(Monitor::new)
  }
}
