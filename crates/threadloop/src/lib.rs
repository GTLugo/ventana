use {
  self::{
    context::{
      Context,
      State,
      ThreadContext,
      ThreadHandler,
      ThreadState,
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
  state: State,
  server_handle: Mutex<Option<JoinHandle<Result<()>>>>,
}

impl<H: ThreadHandler + Send + Sync + 'static> Drop for ThreadLoop<H> {
  fn drop(&mut self) {
    if let Err(e) = self.stop() {
      log::error!("{e}");
    };
  }
}

impl<H: ThreadHandler + Send + Sync + 'static> ThreadLoop<H> {
  pub fn new(handler: Arc<H>) -> Result<Arc<Self>> {
    let (to_client, from_server) = std::sync::mpsc::channel();
    let (to_server, from_client) = std::sync::mpsc::channel();
    let responses = Arc::new(ResponseStore::new());

    let ctx = Context {
      to_client,
      from_client,
      responses: responses.clone(),
      state: Default::default(),
      _ready: PhantomData,
    };

    let server = handler.clone();
    let server_handle = std::thread::Builder::new()
      .name("server".to_string())
      .spawn(move || server_main(server, ctx))?;
    let state = State::default();

    Ok(Arc::new(Self {
      handler,
      from_server: Mutex::new(from_server),
      to_server,
      responses,
      backlog: Mutex::new(Default::default()),
      pending_ack: Mutex::new(None),
      state: state.clone(),
      server_handle: Mutex::new(Some(server_handle)),
    }))
  }

  pub fn start(&self, params: H::Start) -> Result<H::Ready> {
    let from_server = self.from_server.lock().unwrap();
    self
      .to_server
      .send(ClientToServer::start(params))
      .map_err(|e| crate::Error::Disconnected(e.to_string()))?;
    // let response = self.send_request().unwrap();

    loop {
      match from_server.recv().map_err(|e| Error::Disconnected(format!("{e}")))? {
        ServerToClient::Ready(r) => break r,
        // ServerToClient::Ready(Err(e)) => return Err(e),
        ServerToClient::Event(ev) => self.backlog.lock().unwrap().push_back(ev),
        ServerToClient::Stop => return Err(Error::Other("Stop was sent before ThreadLoop was ready.".to_string())),
      }
    }
  }

  // pub fn ready_response(&self) -> &H::Ready {
  //   &self.ready
  // }

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

  pub fn try_send_request(&self, request: ClientToServer<H::Request, H::Start>) -> Result<()> {
    self
      .to_server
      .send(request)
      .map_err(|e| crate::Error::Disconnected(e.to_string()))?;
    self.handler.wake()
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
          return None;
        },
        _ => (), // Ready
      }
    }
  }

  #[inline(always)]
  pub fn set_active(&self) {
    self.state.change_state(ThreadState::Ready);
  }

  #[inline(always)]
  pub fn set_inactive(&self) {
    self.state.change_state(ThreadState::Inactive);
  }

  pub fn stop(&self) -> Result<()> {
    if matches!(self.state.swap_state(ThreadState::Stopping), ThreadState::Stopping | ThreadState::Stopped) {
      return Err(Error::Ignored);
    }

    log::trace!("Stopping server thread");

    self.ack_last_event();
    self.try_send_request(ClientToServer::stop())?;

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

fn server_main<H: ThreadHandler>(server: Arc<H>, ctx: ThreadContext<H>) -> Result<()> {
  log::trace!("Starting server thread; waiting for ClientToServer::Start.");

  let ClientToServer::Start { params, .. } = ctx
    .from_client
    .recv()
    .map_err(|e| crate::Error::Disconnected(e.to_string()))?
  else {
    unreachable!("First command should always be ClientToServer::Start");
  };

  log::trace!("Received ClientToServer::Start; running server.");

  server.run(params, Arc::new(ctx))
}
