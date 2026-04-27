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
      Mutex,
      mpsc::{
        Receiver,
        Sender,
      },
    },
  },
};

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum ThreadState {
  #[default]
  Initializing = 0,
  Ready = 1,
  Inactive = 2,
  Stopping = 3,
  Stopped = 4,
}

impl From<u8> for ThreadState {
  #[inline(always)]
  fn from(value: u8) -> Self {
    match value {
      0 => ThreadState::Initializing,
      1 => ThreadState::Ready,
      2 => ThreadState::Inactive,
      3 => ThreadState::Stopping,
      4 => ThreadState::Stopped,
      _ => unreachable!(), // Yeah, I'm being a little naughty here
    }
  }
}

#[derive(Debug, Clone, Default)]
pub struct State(Arc<Mutex<ThreadState>>);

impl State {
  #[inline(always)]
  pub fn change_state(&self, new: ThreadState) {
    // self.0.store(new as u8, Ordering::Release);
    *self.0.lock().unwrap() = new;
  }

  #[inline(always)]
  pub fn swap_state(&self, new: ThreadState) -> ThreadState {
    let val = &mut self.0.lock().unwrap();
    std::mem::replace(val, new)
  }

  #[inline(always)]
  pub fn current_state(&self) -> ThreadState {
    *self.0.lock().unwrap()
  }

  #[inline(always)]
  pub fn is_ready(&self) -> bool {
    matches!(self.current_state(), ThreadState::Ready)
  }

  #[inline(always)]
  pub fn is_inactive(&self) -> bool {
    matches!(self.current_state(), ThreadState::Inactive)
  }

  #[inline(always)]
  pub fn is_stopping(&self) -> bool {
    matches!(self.current_state(), ThreadState::Stopping | ThreadState::Stopped)
  }

  #[inline(always)]
  pub fn has_stopped(&self) -> bool {
    matches!(self.current_state(), ThreadState::Stopped)
  }
}

pub type ThreadContext<H> = Context<
  <H as ThreadHandler>::Event,
  <H as ThreadHandler>::Request,
  <H as ThreadHandler>::Response,
  <H as ThreadHandler>::Start,
  <H as ThreadHandler>::Ready,
>;

pub trait ThreadHandler {
  type Event: Send + 'static;
  type Request: Send + 'static;
  type Response: Send + 'static;
  type Start: Send + 'static;
  type Ready: Send + 'static;

  fn run(&self, params: Self::Start, ctx: Arc<ThreadContext<Self>>) -> Result<()>;
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
  pub(crate) state: State,
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
    self.state.change_state(ThreadState::Ready);
    let _ = self.to_client.send(ServerToClient::Ready(ready));
  }

  fn try_recv_request(&self) -> Option<ClientToServer<Req, Start>> {
    self.from_client.try_recv().ok()
  }

  pub fn try_handle_request(&self, mut handler: impl FnMut(ClientToServer<Req, Start>) -> Result<Res>) -> Result<bool> {
    if self.state.has_stopped() {
      return Ok(false);
    }

    if let Some(request) = self.try_recv_request() {
      if let ClientToServer::Stop { .. } = &request {
        self.state.change_state(ThreadState::Stopped);
      }

      self.responses.insert_and_notify(request.id(), handler(request)?);
    } else {
      return Ok(false);
    }

    Ok(true)
  }

  pub fn send_event(&self, event: E) -> Result<()> {
    let ack = self.state.is_ready().then(AcknowledgeSignal::new);
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

  #[inline(always)]
  pub fn set_active(&self) {
    self.state.change_state(ThreadState::Ready);
  }

  #[inline(always)]
  pub fn set_inactive(&self) {
    self.state.change_state(ThreadState::Inactive);
  }

  #[inline(always)]
  pub fn current_state(&self) -> ThreadState {
    self.state.current_state()
  }

  #[inline(always)]
  pub fn is_ready(&self) -> bool {
    self.state.is_ready()
  }

  #[inline(always)]
  pub fn is_inactive(&self) -> bool {
    self.state.is_inactive()
  }

  #[inline(always)]
  pub fn is_stopping(&self) -> bool {
    self.state.is_stopping()
  }

  #[inline(always)]
  pub fn has_stopped(&self) -> bool {
    self.state.has_stopped()
  }

  pub fn notify_stopped(&self) -> Result<()> {
    self.set_inactive();
    self
      .to_client
      .send(ServerToClient::Stop)
      .map_err(|e| crate::Error::Disconnected(e.to_string()))
  }
}
