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
  fn instance() -> &'static Self
  where
    Self: Sized;

  fn name(&self) -> &'static str;

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError>;

  fn list_available_monitors(&self) -> VecDeque<Arc<dyn BackendMonitor>>;

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError>;
}
