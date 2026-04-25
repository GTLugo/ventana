use {
  crate::{
    Result,
    message::{
      ClientToServer,
      Envelope,
      ResponseStore,
      ServerToClient,
    },
    signal::AcknowledgeSignal,
  },
  std::{
    marker::PhantomData,
    sync::{
      Arc,
      atomic::{
        AtomicBool,
        Ordering,
      },
      mpsc::{
        Receiver,
        Sender,
      },
    },
  },
};

pub trait ThreadHandler {
  type Event: Send + 'static;
  type Request: Send + 'static;
  type Response: Send + 'static;
  type Start: Send + 'static;
  type Ready: Send + 'static;

  #[allow(clippy::type_complexity)]
  fn run(
    &self,
    params: Self::Start,
    ctx: Arc<Context<Self::Event, Self::Request, Self::Response, Self::Start, Self::Ready>>,
  ) -> Result<()>;
  fn wake(&self) -> Result<()>;
}

pub struct Context<E, Req, Res, Start, Ready>
where
  E: Send + 'static,
  Req: Send + 'static,
  Res: Send + 'static,
  Start: Send + 'static,
  Ready: Send + 'static,
{
  pub(crate) to_client: Sender<ServerToClient<E, Ready>>,
  pub(crate) from_client: Receiver<ClientToServer<Req, Start>>,
  pub(crate) responses: Arc<ResponseStore<Res>>,
  pub(crate) stopped: Arc<AtomicBool>,
  pub(crate) _ready: PhantomData<Ready>,
}

impl<E, Req, Res, Start, Ready> Context<E, Req, Res, Start, Ready>
where
  E: Send + 'static,
  Req: Send + 'static,
  Res: Send + 'static,
  Start: Send + 'static,
  Ready: Send + 'static,
{
  pub fn signal_ready(&self, ready: Result<Ready>) {
    let _ = self.to_client.send(ServerToClient::Ready(ready));
  }

  fn try_recv_request(&self) -> Option<ClientToServer<Req, Start>> {
    self.from_client.try_recv().ok()
  }

  pub fn try_handle_request(&self, mut handler: impl FnMut(ClientToServer<Req, Start>) -> Result<Res>) -> Result<bool> {
    if let Some(request) = self.try_recv_request() {
      self.responses.insert_and_notify(request.id(), handler(request)?);
    } else {
      return Ok(false);
    }
    Ok(true)
  }

  pub fn send_event(&self, event: E) -> Result<()> {
    let ack = Some(AcknowledgeSignal::new());
    self
      .to_client
      .send(ServerToClient::Event(Envelope {
        ack: ack.clone(),
        message: event,
      }))
      .map_err(|e| crate::Error::Disconnected(e.to_string()))?;
    if let Some(ack) = ack {
      ack.wait();
    }
    Ok(())
  }

  pub fn notify_stopped(&self) -> Result<()> {
    self.stopped.store(true, Ordering::Release);
    self
      .to_client
      .send(ServerToClient::Stop)
      .map_err(|e| crate::Error::Disconnected(e.to_string()))
  }
}
