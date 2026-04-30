use {
  crate::{
    context::ThreadHandler,
    message::{
      ClientToServer,
      Id,
    },
  },
  std::sync::Arc,
};

// Mostly unused for now. part of the big v2 plan
#[allow(unused)]
#[derive(Clone)]
pub struct ThreadLoopProxy<H>
where
  Self: Send + Sync,
  H: ThreadHandler + Send + Sync + 'static,
{
  handler: Arc<H>,
}

impl<H> ThreadLoopProxy<H>
where
  Self: Send + Sync,
  H: ThreadHandler + Send + Sync + 'static,
{
  pub fn new(handler: Arc<H>) -> Self {
    Self { handler }
  }

  pub fn send_request(&self, request: ClientToServer<H::Request>) -> crate::Result<Id> {
    let id = request.id();
    self.handler.send(request)?;
    Ok(id)
  }
}
