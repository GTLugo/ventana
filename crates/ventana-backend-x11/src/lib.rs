mod backend;
mod monitor;
mod window;

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

#[derive(Clone)]
pub struct X11(
  #[cfg(all(
    unix,
    not(any(
      target_os = "redox",
      target_family = "wasm",
      target_os = "android",
      target_vendor = "apple"
    ))
  ))]
  Arc<crate::backend::X11State>,
);

impl Backend for X11 {
  fn instance() -> Option<&'static Self>
  where
    Self: Sized,
  {
    #[cfg(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    ))]
    {
      static INSTANCE: std::sync::LazyLock<Option<X11>> = std::sync::LazyLock::new(X11::new);
      INSTANCE.as_ref()
    }
  }

  fn is_available() -> bool
  where
    Self: Sized,
  {
    #[cfg(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    ))]
    {
      backend::is_available()
    }
    #[cfg(not(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    )))]
    false
  }

  fn name(&self) -> &'static str {
    "X11"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    #[cfg(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    ))]
    {
      backend::create_window(settings)
    }
    #[cfg(not(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    )))]
    Err(RequestError::NotSupported("X11 backend is not supported"))
  }

  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    #[cfg(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    ))]
    {
      backend::list_available_monitors()
    }
    #[cfg(not(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    )))]
    Err(RequestError::NotSupported("X11 backend is not supported"))
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    #[cfg(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    ))]
    {
      backend::primary_monitor()
    }
    #[cfg(not(all(
      unix,
      not(any(
        target_os = "redox",
        target_family = "wasm",
        target_os = "android",
        target_vendor = "apple"
      ))
    )))]
    Err(RequestError::NotSupported("X11 backend is not supported"))
  }
}
