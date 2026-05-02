pub mod backend;

use {
  backend::{
    appkit::AppKit,
    linux::Linux,
    win32::Win32,
  },
  backend_wayland::Wayland,
  backend_x11::X11,
  hal::{
    backend::Backend as HalBackend,
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
      Mutex,
    },
  },
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Backend {
  #[default]
  Win32 = 0,
  AppKit = 1,
  Wayland = 2,
  X11 = 3,
  Web = 4,
  Android = 5,
  UIKit = 6,
}

impl Backend {
  pub fn backend(self) -> Option<&'static dyn HalBackend> {
    match self {
      Backend::Win32 => Win32::backend(),
      Backend::AppKit => AppKit::backend(),
      Backend::Wayland => Wayland::backend(),
      Backend::X11 => X11::backend(),
      Backend::Web => unimplemented!(),
      Backend::Android => unimplemented!(),
      Backend::UIKit => unimplemented!(),
    }
  }

  pub fn is_available(&self) -> bool {
    match self {
      Backend::Win32 => Win32::is_available(),
      Backend::AppKit => AppKit::is_available(),
      Backend::Wayland => Wayland::is_available(),
      Backend::X11 => X11::is_available(),
      Backend::Web => false,
      Backend::Android => false,
      Backend::UIKit => false,
    }
  }
}

static PREFERENCES: LazyLock<Mutex<&'static [Backend]>> = LazyLock::new(|| {
  Mutex::new(&[
    Backend::Win32,
    Backend::AppKit,
    Backend::Wayland,
    Backend::X11,
    Backend::Web,
    Backend::Android,
    Backend::UIKit,
  ])
});

#[derive(Clone)]
pub struct AutoBackend(&'static dyn HalBackend);

impl Debug for AutoBackend {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.name())
  }
}

impl HalBackend for AutoBackend {
  fn instance() -> Option<&'static Self>
  where
    Self: Sized,
  {
    static INSTANCE: LazyLock<Option<AutoBackend>> =
      LazyLock::new(|| AutoBackend::auto().ok().map(AutoBackend));
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
  fn auto() -> Result<&'static dyn HalBackend, RequestError> {
    let Some(backend) = PREFERENCES.lock().unwrap().iter().find(|b| b.is_available()) else {
      return Err(RequestError::not_supported("no supported backend available to auto-select from"));
    };
    backend.backend().ok_or(RequestError::not_supported("failed to initialize backend"))
  }

  /// Sets which backends the AutoBackend will select from and in what order they are selected.
  ///
  /// Default: `[Win32, AppKit, Wayland, X11, Web, Android, UIKit,]`
  pub fn set_preferences(preferences: &'static [Backend]) {
    *PREFERENCES.lock().unwrap() = preferences;
  }
}
