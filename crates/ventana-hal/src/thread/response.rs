use {
  super::command::CommandId,
  std::{
    collections::HashMap,
    sync::{
      Arc,
      Mutex,
      MutexGuard,
    },
  },
};

#[derive(Debug, Clone)]
pub struct ResponseEnvelope<ThreadResponse> {
  pub id: CommandId,
  pub response: ThreadResponse,
}

#[derive(Debug)]
pub struct Responses<ThreadResponse>(Arc<Mutex<HashMap<CommandId, ThreadResponse>>>)
where
  Self: Send + Sync,
  ThreadResponse: Send + Sync + 'static;

// Not sure why this couldn't be inferred, but likely some sort of weird generics issue.
impl<ThreadResponse> Clone for Responses<ThreadResponse>
where
  ThreadResponse: Send + Sync + 'static,
{
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<ThreadResponse> Default for Responses<ThreadResponse>
where
  ThreadResponse: Send + Sync + 'static,
{
  fn default() -> Self {
    Self(Default::default())
  }
}

impl<ThreadResponse> Responses<ThreadResponse>
where
  ThreadResponse: Send + Sync + 'static,
{
  pub fn new() -> Self {
    Default::default()
  }

  fn map_lock(&self) -> MutexGuard<'_, HashMap<CommandId, ThreadResponse>> {
    self.0.lock().unwrap()
  }

  pub fn insert(&self, ResponseEnvelope { id, response }: ResponseEnvelope<ThreadResponse>) {
    self.map_lock().insert(id, response);
  }

  pub fn remove(&self, id: CommandId) -> Option<ThreadResponse> {
    self.map_lock().remove(&id)
  }
}
