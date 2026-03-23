use {
  crate::{
    error::RequestError,
    settings::WindowSettings,
    window::BackendWindow,
  },
  keyboard_types::Code,
  std::sync::Arc,
};

pub trait Backend: Send + Sync + 'static {
  fn instance() -> Arc<dyn Backend>
  where
    Self: Sized;

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError>;

  fn key_to_scancode(&self, key: Code) -> Option<u32>;

  fn scancode_to_key(&self, scancode: u32) -> Code;
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
