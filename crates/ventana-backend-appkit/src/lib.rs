mod backend;
mod event;
mod window;

#[allow(unused)]
use {
  std::{
    collections::VecDeque,
    sync::{
      Arc,
      LazyLock,
    },
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
pub struct AppKit;

impl Backend for AppKit {
  #[cfg(target_os = "macos")]
  fn instance() -> Option<&'static Self>
  where
    Self: Sized,
  {
    static INSTANCE: LazyLock<Option<AppKit>> = LazyLock::new(AppKit::new);
    INSTANCE.as_ref()
  }

  #[cfg(target_os = "macos")]
  fn is_available() -> bool
  where
    Self: Sized,
  {
    true
  }

  fn name(&self) -> &'static str {
    "AppKit"
  }

  #[cfg(target_os = "macos")]
  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(self::window::AppKitWindow::new(settings)?))
  }

  #[cfg(target_os = "macos")]
  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    todo!()
  }

  #[cfg(target_os = "macos")]
  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    todo!()
  }
}
