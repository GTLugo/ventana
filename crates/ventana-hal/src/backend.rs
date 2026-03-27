use {
  crate::{
    error::RequestError,
    settings::WindowSettings,
    window::BackendWindow,
  },
  std::sync::Arc,
};

pub trait Backend: Send + Sync {
  fn instance() -> Arc<dyn Backend>
  where
    Self: Sized;

  fn name(&self) -> &'static str;

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError>;
}

// #[derive(Clone)]
// pub struct Context {
//   backend: Arc<dyn Backend>,
// }
//
// impl<T: Backend> From<T> for Context {
//   fn from(backend: T) -> Self {
//     Self {
//       backend: Arc::new(backend),
//     }
//   }
// }
//
// impl Deref for Context {
//   type Target = dyn Backend;
//
//   fn deref(&self) -> &Self::Target {
//     self.backend.as_ref()
//   }
// }
