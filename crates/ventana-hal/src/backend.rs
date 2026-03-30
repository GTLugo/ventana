use {
  crate::{
    error::RequestError,
    monitor::BackendMonitor,
    settings::WindowSettings,
    window::BackendWindow,
  },
  std::sync::Arc,
};

pub trait Backend: Send + Sync {
  fn instance() -> impl Backend
  where
    Self: Sized;

  fn name(&self) -> &'static str;

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError>;

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError>;
}
