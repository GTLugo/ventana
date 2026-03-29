use {
  crate::{
    error::RequestError,
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
}
