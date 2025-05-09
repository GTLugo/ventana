use std::{ops::Deref, sync::Arc};

use crate::provider::{InputProvider, WindowProvider};

pub trait Backend: WindowProvider + InputProvider + 'static {}

#[derive(Clone)]
pub struct Context {
  backend: Arc<dyn Backend>,
}

impl<T: Backend> From<T> for Context {
  fn from(backend: T) -> Self {
    Self {
      backend: Arc::new(backend),
    }
  }
}

impl Deref for Context {
  type Target = dyn Backend;

  fn deref(&self) -> &Self::Target {
    self.backend.as_ref()
  }
}
