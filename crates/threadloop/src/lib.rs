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
      ServerToClient,
    },
    proxy::ThreadLoopProxy,
    signal::AcknowledgeSignal,
  },
  crossbeam_channel::{
    Receiver,
    TryRecvError,
  },
  std::{
    sync::{
      Arc,
      Mutex,
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
  from_server: Receiver<ServerToClient<H::Event, H::Ready>>,
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
    let (to_client, from_server) = crossbeam_channel::unbounded();
    let proxy = ThreadLoopProxy::new(handler.clone());

    let ctx = Context { to_client, state: Default::default() };

    let server_handle = std::thread::Builder::new()
      .name("server".to_string())
      .spawn(move || server_main(handler, ctx, params))?;
    let state = State::default();

    Ok(Arc::new(Self {
      proxy,
      from_server,
      pending_ack: Mutex::new(None),
      state: state.clone(),
      server_handle: Mutex::new(Some(server_handle)),
    }))
  }

  pub fn start(&self) -> Result<H::Ready> {
    for event in self.from_server.iter() {
      if let ServerToClient::Ready(r) = event {
        return r;
      }
    }
    Err(Error::Disconnected("Threadloop disconnected before it was ready.".to_string()))
  }

  pub fn proxy(&self) -> &ThreadLoopProxy<H> {
    &self.proxy
  }

  #[inline]
  fn poll_event(
    &self,
    wait: bool,
  ) -> std::result::Result<ServerToClient<H::Event, H::Ready>, NextEventError> {
    match wait {
      true => self.from_server.recv().map_err(|_| NextEventError::Disconnected),
      false => self.from_server.try_recv().map_err(|e| match e {
        TryRecvError::Empty => NextEventError::Empty,
        TryRecvError::Disconnected => NextEventError::Disconnected,
      }),
    }
  }

  pub fn next_event(&self, wait: bool) -> std::result::Result<H::Event, NextEventError> {
    self.ack_last_event();
    match self.poll_event(wait) {
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
    self.proxy.send_request(ClientToServer::stop())?;

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

  let ctx = Arc::new(ctx);

  ctx.signal_ready(server.start(params, ctx.clone()));

  log::trace!("Running server.");

  server.run(ctx)
}
