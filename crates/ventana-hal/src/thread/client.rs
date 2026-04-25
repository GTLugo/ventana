use {
  super::{
    command::CommandEnvelope,
    event::{
      EventEnvelope,
      // ThreadEvent,
    },
    // response::ResponseEnvelope,
    signal::StopSignal,
  },
  crate::{
    error::RequestError,
    event::Event,
    thread::{
      response::Responses,
      signal::AcknowledgeSignal,
    },
    types::Flow,
  },
  crossbeam_channel::{
    Receiver,
    Sender,
    TryRecvError,
  },
  std::sync::{
    Mutex,
    RwLock,
  },
};
// pub type DropHandler<Command, ThreadResponse> =
//   Box<dyn Fn(&Client<Command, ThreadResponse>) + Send + Sync + 'static>;

// pub type WakeHandler<Window> = Box<dyn Fn(&Window) + Send + Sync + 'static>;

pub struct ClientWrapper<Command, ThreadResponse, C>
where
  Self: Send + Sync + 'static,
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
  C: Client<Command = Command, ThreadResponse = ThreadResponse>,
{
  client: C,

  event_rx: Receiver<EventEnvelope>,
  message_tx: Sender<CommandEnvelope<Command>>,

  // event_backlog: SegQueue<EventEnvelope>,
  pending_ack: Mutex<Option<AcknowledgeSignal>>,

  responses: Responses<ThreadResponse>,

  stop_signal: StopSignal,
  flow: RwLock<Flow>,
}

impl<Command, ThreadResponse, C> Drop for ClientWrapper<Command, ThreadResponse, C>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
  C: Client<Command = Command, ThreadResponse = ThreadResponse>,
{
  fn drop(&mut self) {
    log::trace!("Dropping Client");
  }
}

impl<Command, ThreadResponse, C> ClientWrapper<Command, ThreadResponse, C>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
  C: Client<Command = Command, ThreadResponse = ThreadResponse>,
{
  pub fn new(
    client: C,
    stop_signal: StopSignal,
    flow: Flow,
    event_rx: Receiver<EventEnvelope>,
    message_tx: Sender<CommandEnvelope<Command>>,
    responses: Responses<ThreadResponse>,
  ) -> Self {
    Self {
      client,
      event_rx,
      message_tx,
      // event_backlog: SegQueue::new(),
      pending_ack: Mutex::new(None),
      responses,
      stop_signal,
      flow: RwLock::new(flow),
    }
  }

  fn wake_server(&self) {
    if let Err(error) = self.client.wake(self) {
      log::error!("{error}");
    }
    // self.server_thread.wake();
  }

  pub fn client(&self) -> &C {
    &self.client
  }

  pub fn client_mut(&mut self) -> &mut C {
    &mut self.client
  }

  pub fn destroy(&mut self) {
    self.acknowledge_last_event();
    if let Err(error) = self.client.destroy(self) {
      log::error!("{error}");
    }
    // self.server_thread.destroy();
  }

  pub fn set_flow(&self, flow: Flow) {
    *self.flow.write().unwrap() = flow;
  }

  pub fn stop_signal(&self) -> StopSignal {
    self.stop_signal.clone()
  }

  fn send(&self, message: Command, wake_server: bool, wait_for_response: bool) -> Option<ThreadResponse> {
    let ack = wait_for_response.then(AcknowledgeSignal::new);
    let envelope = CommandEnvelope::new(message);
    let id = envelope.id;
    log::trace!("Sending command `{id:?}` | wake_server: {wake_server}, wait_for_response: {wait_for_response}");
    self.message_tx.try_send(envelope).ok()?; // try_send
    if wake_server {
      self.wake_server();
    }
    if let Some(ack) = ack {
      log::trace!("Waiting for command ack");
      ack.wait();
    }
    self.responses.remove(id)
  }

  pub fn send_request(&self, message: Command, wake_server: bool) -> Option<ThreadResponse> {
    self.send(message, wake_server, true)
  }

  pub fn send_command(&self, message: Command, wake_server: bool) {
    let _ = self.send(message, wake_server, false);
  }

  pub fn next_event(&self) -> Option<Event> {
    self.acknowledge_last_event();

    if self.stop_signal.should_stop() {
      log::trace!("STOP_SIGNAL | Stop signal triggered; returning None");
      return None;
    }

    // self.pump_until_first_event()

    let EventEnvelope { event, ack } = self.receive_event()?;

    if let Some(ack) = ack {
      self.pending_ack.lock().unwrap().replace(ack);
    }

    Some(event)
  }

  fn receive_event(&self) -> Option<EventEnvelope> {
    let flow = *self.flow.read().unwrap();
    match flow {
      Flow::Wait => self.event_rx.recv().ok(),
      Flow::Poll => match self.event_rx.try_recv() {
        Ok(event) => Some(event),
        Err(TryRecvError::Empty) => Some(EventEnvelope::empty()),
        Err(TryRecvError::Disconnected) => None,
      },
    }
  }

  // fn pump_until_first_event(&self) -> Option<Event> {
  //   loop {
  //     if let Some(EventEnvelope { event, ack }) = self.event_backlog.pop() {
  //       log::trace!("NEXT_EVENT | Popping event `{event:?}` from backlog | ack: {:?}", ack.is_some());
  //       if let Some(ack) = ack {
  //         self.pending_ack.lock().unwrap().replace(ack);
  //       }

  //       return Some(event);
  //     }

  //     match self.receive_event() {
  //       None => (),
  //       Some(ThreadEvent::Event(event)) => {
  //         log::trace!("NEXT_EVENT | Pushing event `{event:?}` to backlog | ack: {:?}", event.ack.is_some());
  //         self.event_backlog.push(event);
  //       },
  //       Some(ThreadEvent::CommandResponse(response)) => {
  //         log::trace!("NEXT_EVENT | Inserting response");
  //         self.responses.insert(response);
  //       },
  //     }
  //   }
  // }

  fn acknowledge_last_event(&self) {
    if let Some(ack) = self.pending_ack.lock().unwrap().take() {
      log::trace!("Acknowledging last event");
      ack.heard();
    }
  }

  // fn pump_while_waiting_for_response(&self, id: CommandId) -> Option<ThreadResponse> {
  //   log::trace!("Waiting for response for `{id:?}`");
  //   loop {
  //     if let Some(response) = self.responses.lock().ok()?.remove(&id) {
  //       // log::debug!("Returning response for `{id:?}`");
  //       return Some(response);
  //     }

  //     log::warn!("No response; beginning event pump");

  //     let event = self.event_rx.recv();

  //     // if let Err(TryRecvError::Empty) = &event {
  //     //   // log::warn!("No events");
  //     //   continue; // return or continue?
  //     // }

  //     // TODO: What the fuck is breaking here???? GO LINE BY LINE

  //     match event.ok()? {
  //       ThreadEvent::Event(EventEnvelope { event, ack }) => {
  //         log::trace!("SEND_COMMAND | Pushing event `{event:?}` to backlog | ack: {:?}", ack.is_some());
  //         self.event_backlog.push(EventEnvelope { event, ack: None });
  //         if let Some(ack) = ack {
  //           ack.heard();
  //         }
  //       },
  //       ThreadEvent::CommandResponse(ResponseEnvelope { id, response }) => {
  //         log::trace!("SEND_COMMAND | Inserting response");
  //         self
  //           .responses
  //           .lock()
  //           .ok()?
  //           // .map_err(|e| os_error_fmt!("{e}"))?
  //           .insert(id, response);
  //       },
  //     }
  //   }
  // }
}

pub trait Client: Send + Sync {
  type Command: Send + Sync;
  type ThreadResponse: Send + Sync;

  // fn run(&self) -> Result<(), RequestError>;
  fn wake(&self, client: &ClientWrapper<Self::Command, Self::ThreadResponse, Self>) -> Result<(), RequestError>
  where
    Self: Sized;
  fn destroy(&self, client: &ClientWrapper<Self::Command, Self::ThreadResponse, Self>) -> Result<(), RequestError>
  where
    Self: Sized;
}
