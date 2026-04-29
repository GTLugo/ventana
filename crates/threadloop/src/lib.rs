pub mod context;
pub mod message;
pub mod proxy;
pub mod signal;
pub mod v2;

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
    proxy::ThreadLoopProxy,
    signal::AcknowledgeSignal,
  },
  std::{
    collections::LinkedList,
    sync::{
      Arc,
      Mutex,
      mpsc::{
        Receiver,
        TryRecvError,
      },
    },
    thread::JoinHandle,
  },
};

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

#[derive(Debug, thiserror::Error)]
pub enum NextEventError {
  #[error("no new events available")]
  Empty,
  #[error("server disconnected")]
  Disconnected,
}

pub type Result<T> = std::result::Result<T, Error>;

pub struct ThreadLoop<H>
where
  H: ThreadHandler + Send + Sync + 'static,
{
  proxy: ThreadLoopProxy<H>,
  from_server: Mutex<Receiver<ServerToClient<H::Event, H::Ready>>>,
  backlog: Mutex<LinkedList<Envelope<H::Event>>>,
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
  pub fn new(handler: Arc<H>, params: H::Start) -> Result<Arc<Self>> {
    let (to_client, from_server) = std::sync::mpsc::channel();
    let responses = Arc::new(ResponseStore::new());
    let proxy = ThreadLoopProxy::new(handler.clone(), responses.clone());

    let ctx = Context { to_client, responses: responses.clone(), state: Default::default() };

    let server_handle = std::thread::Builder::new()
      .name("server".to_string())
      .spawn(move || server_main(handler, ctx, params))?;
    let state = State::default();

    Ok(Arc::new(Self {
      from_server: Mutex::new(from_server),
      proxy,
      backlog: Mutex::new(Default::default()),
      pending_ack: Mutex::new(None),
      state: state.clone(),
      server_handle: Mutex::new(Some(server_handle)),
    }))
  }

  pub fn start(&self) -> Result<H::Ready> {
    let from_server = self.from_server.lock().unwrap();

    loop {
      match from_server.recv().map_err(|e| Error::Disconnected(format!("{e}")))? {
        ServerToClient::Ready(r) => break r,
        // ServerToClient::Ready(Err(e)) => return Err(e),
        ServerToClient::Event(ev) => self.backlog.lock().unwrap().push_back(ev),
        ServerToClient::Stop => {
          return Err(Error::Other("Stop was sent before ThreadLoop was ready.".to_string()));
        },
      }
    }
  }

  pub fn proxy(&self) -> &ThreadLoopProxy<H> {
    &self.proxy
  }

  #[inline]
  fn drain_backlog(&self) -> Option<H::Event> {
    match self.backlog.lock().unwrap().pop_front() {
      Some(Envelope { message, ack }) => {
        *self.pending_ack.lock().unwrap() = ack;
        Some(message)
      },
      _ => None,
    }
  }

  #[inline]
  fn poll_event(
    &self,
    wait: bool,
  ) -> std::result::Result<ServerToClient<H::Event, H::Ready>, NextEventError> {
    match wait {
      true => self.from_server.lock().unwrap().recv().map_err(|_| NextEventError::Disconnected),
      false => self.from_server.lock().unwrap().try_recv().map_err(|e| match e {
        TryRecvError::Empty => NextEventError::Empty,
        TryRecvError::Disconnected => NextEventError::Disconnected,
      }),
    }
  }

  fn are_events_in_backlog(&self) -> bool {
    // this is mostly to 110% guarantee the lock is dropped. Maybe unnecessary, but I am paranoid
    !self.backlog.lock().unwrap().is_empty()
  }

  pub fn next_event(&self, wait: bool) -> std::result::Result<H::Event, NextEventError> {
    self.ack_last_event();

    while self.are_events_in_backlog() {
      if let Some(event) = self.drain_backlog() {
        return Ok(event);
      }
    }

    let event = self.poll_event(wait);

    match event {
      Ok(ServerToClient::Ready(_)) => Err(NextEventError::Empty),
      Ok(ServerToClient::Event(Envelope { message, ack })) => {
        *self.pending_ack.lock().unwrap() = ack;
        Ok(message)
      },
      Ok(ServerToClient::Stop) => Err(NextEventError::Disconnected),
      Err(error) => Err(error), // Ready
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
    self.proxy.try_send_request(ClientToServer::stop())?;

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

fn server_main<H: ThreadHandler>(server: Arc<H>, ctx: ThreadContext<H>, params: H::Start) -> Result<()> {
  log::trace!("Starting server thread.");

  // log::trace!("Received ClientToServer::Start; starting server.");

  let ctx = Arc::new(ctx);

  ctx.signal_ready(server.start(params, ctx.clone()));

  log::trace!("Running server.");

  server.run(ctx)
}
