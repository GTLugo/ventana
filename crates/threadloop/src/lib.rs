use {
  self::{
    context::{
      Context,
      ThreadHandler,
    },
    message::{
      ClientToServer,
      Envelope,
      ResponseStore,
      ServerToClient,
    },
    signal::AcknowledgeSignal,
  },
  std::{
    collections::VecDeque,
    marker::PhantomData,
    sync::{
      Arc,
      Mutex,
      atomic::{
        AtomicBool,
        Ordering,
      },
      mpsc::{
        Receiver,
        Sender,
      },
    },
    thread::JoinHandle,
  },
};

pub mod context;
pub mod message;
pub mod signal;

#[derive(Debug, thiserror::Error)]
pub enum Error {
  #[error("System error: `{0}`")]
  OS(#[from] std::io::Error),
  #[error("System ignored the action")]
  Ignored,
  #[error("`{0}`")]
  Other(String),
  #[error("Channel disconnected: `{0}`")]
  Disconnected(String),
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct ThreadLoop<H>
where
  H: ThreadHandler + Send + Sync + 'static,
{
  handler: Arc<H>,
  from_server: Mutex<Receiver<ServerToClient<H::Event, H::Ready>>>,
  to_server: Sender<ClientToServer<H::Request, H::Start>>,
  responses: Arc<ResponseStore<H::Response>>,
  backlog: Mutex<VecDeque<Envelope<H::Event>>>,
  pending_ack: Mutex<Option<AcknowledgeSignal>>,
  ready: H::Ready,
  stopped: Arc<AtomicBool>,
  server_handle: Mutex<Option<JoinHandle<Result<()>>>>,
}

impl<H: ThreadHandler + Send + Sync + 'static> Drop for ThreadLoop<H> {
  fn drop(&mut self) {
    if let Err(e) = self.shutdown() {
      log::error!("{e}");
    };
  }
}

impl<H: ThreadHandler + Send + Sync + 'static> ThreadLoop<H> {
  pub fn new(handler: Arc<H>) -> Result<Arc<Self>> {
    let (to_client, from_server) = std::sync::mpsc::channel();
    let (to_server, from_client) = std::sync::mpsc::channel();
    let responses = Arc::new(ResponseStore::new());
    let stopped = Arc::new(AtomicBool::new(false));

    let ctx = Context {
      to_client,
      from_client,
      responses: responses.clone(),
      stopped: stopped.clone(),
      _ready: PhantomData,
    };

    let server = handler.clone();
    let server_handle = std::thread::Builder::new().name("server".to_string()).spawn(move || {
      let ClientToServer::Start { params, .. } = ctx
        .from_client
        .recv()
        .map_err(|e| crate::Error::Disconnected(e.to_string()))?
      else {
        unreachable!("First command should always be CreateWindow");
      };
      server.run(params, Arc::new(ctx))
    })?;

    let mut backlog = VecDeque::new();
    let ready = loop {
      match from_server.recv().map_err(|e| Error::Disconnected(format!("{e}")))? {
        ServerToClient::Ready(Ok(r)) => break r,
        ServerToClient::Ready(Err(e)) => return Err(e),
        ServerToClient::Event(ev) => backlog.push_back(ev),
        ServerToClient::Stop => return Err(Error::Other("Stop was sent before ThreadLoop was ready.".to_string())),
      }
    };

    Ok(Arc::new(Self {
      handler,
      from_server: Mutex::new(from_server),
      to_server,
      responses,
      backlog: Mutex::new(backlog),
      pending_ack: Mutex::new(None),
      ready,
      stopped,
      server_handle: Mutex::new(Some(server_handle)),
    }))
  }

  pub fn ready_response(&self) -> &H::Ready {
    &self.ready
  }

  // TODO: Flatten the result into a custom enum
  pub fn send_request(&self, request: ClientToServer<H::Request, H::Start>) -> Result<Option<H::Response>> {
    let id = request.id();
    self
      .to_server
      .send(request)
      .map_err(|e| crate::Error::Disconnected(e.to_string()))?;
    self.handler.wake()?;
    Ok(self.responses.wait_and_take(&id))
  }

  pub fn next_event(&self, wait: bool) -> Option<H::Event> {
    self.ack_last_event();

    loop {
      if let Some(Envelope { message, ack }) = self.backlog.lock().unwrap().pop_front() {
        *self.pending_ack.lock().unwrap() = ack;
        return Some(message);
      }

      let event = match wait {
        true => self.from_server.lock().unwrap().recv().ok()?,
        false => self.from_server.lock().unwrap().try_recv().ok()?,
      };

      match event {
        ServerToClient::Event(Envelope { message, ack }) => {
          *self.pending_ack.lock().unwrap() = ack;
          return Some(message);
        },
        ServerToClient::Stop => {
          self.stopped.store(true, Ordering::Release);
          return None;
        },
        _ => (), // Ready
      }
    }
  }

  pub fn shutdown(&self) -> Result<()> {
    if self.stopped.swap(true, Ordering::AcqRel) {
      return Err(Error::Ignored);
    }

    self.ack_last_event();
    self.send_request(ClientToServer::stop())?;

    let handle = self.server_handle.lock().unwrap().take();
    if let Some(handle) = handle {
      log::trace!("Waiting for server thread to join main thread");
      handle.join().unwrap()?;
      log::trace!("Server thread joined main thread");
    }

    Ok(())
  }

  fn ack_last_event(&self) {
    let pending = self.pending_ack.lock().unwrap().take();
    if let Some(pending) = pending {
      pending.heard();
    }
  }
}
