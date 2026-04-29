use {
  crate::{
    context::ThreadHandler,
    message::{
      ClientToServer,
      ResponseStore,
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
  responses: Arc<ResponseStore<H::Response>>,
}

impl<H> ThreadLoopProxy<H>
where
  Self: Send + Sync,
  H: ThreadHandler + Send + Sync + 'static,
{
  pub fn new(handler: Arc<H>, responses: Arc<ResponseStore<H::Response>>) -> Self {
    Self { handler, responses }
  }

  pub fn send_request(&self, request: ClientToServer<H::Request>) -> crate::Result<Option<H::Response>> {
    let id = request.id();
    self.handler.send(request)?;
    Ok(self.responses.wait_and_take(&id))
  }

  pub fn try_send_request(&self, request: ClientToServer<H::Request>) -> crate::Result<()> {
    self.handler.send(request)
  }
}
