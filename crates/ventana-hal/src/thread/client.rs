use {
  super::{
    acknowledge::AcknowledgementToken,
    command::{
      CommandEnvelope,
      CommandId,
    },
    event::{
      EventEnvelope,
      ThreadEvent,
    },
    response::ResponseEnvelope,
    signal::StopSignal,
  },
  crate::{
    event::Event,
    types::Flow,
  },
  crossbeam_channel::{
    Receiver,
    Sender,
    TryRecvError,
  },
  crossbeam_queue::SegQueue,
  std::{
    collections::HashMap,
    sync::Mutex,
  },
};

pub type DropHandler<Window, Command, ThreadResponse> =
  Box<dyn Fn(&Client<Window, Command, ThreadResponse>) + Send + Sync + 'static>;

pub type WakeHandler<Window> = Box<dyn Fn(&Window) + Send + Sync + 'static>;

pub struct Client<Window, Command, ThreadResponse>
where
  Self: Send + Sync + 'static,
  Window: Send + Sync + 'static,
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  pub window: Option<Window>,

  stop_signal: StopSignal,

  on_drop: Option<DropHandler<Window, Command, ThreadResponse>>,
  on_wake: Option<WakeHandler<Window>>,

  event_rx: Receiver<ThreadEvent<ThreadResponse>>,
  message_tx: Sender<CommandEnvelope<Command>>,

  event_backlog: SegQueue<EventEnvelope>,
  pending_ack: Mutex<Option<AcknowledgementToken>>,

  responses: Mutex<HashMap<CommandId, ThreadResponse>>,

  flow: Mutex<Flow>,
}

impl<Window: Send + Sync + 'static, Command: Send + Sync + 'static, ThreadResponse: Send + Sync + 'static>
  Client<Window, Command, ThreadResponse>
{
  pub fn new(
    stop_signal: StopSignal,
    flow: Flow,
    event_rx: Receiver<ThreadEvent<ThreadResponse>>,
    message_tx: Sender<CommandEnvelope<Command>>,
    on_drop: impl Fn(&Client<Window, Command, ThreadResponse>) + Send + Sync + 'static,
    on_wake: impl Fn(&Window) + Send + Sync + 'static,
  ) -> Self {
    Self {
      window: None,
      stop_signal,
      event_rx,
      message_tx,
      event_backlog: SegQueue::new(),
      pending_ack: Mutex::new(None),
      responses: Mutex::new(HashMap::new()),
      flow: Mutex::new(flow),
      on_drop: Some(Box::new(on_drop)),
      on_wake: Some(Box::new(on_wake)),
    }
  }

  pub fn destroy(&mut self) {
    self.acknowledge_last_event();
    if let Some(on_drop) = self.on_drop.take() {
      on_drop(self);
    }
  }

  pub fn set_flow(&self, flow: Flow) {
    *self.flow.lock().unwrap() = flow;
  }

  pub fn stop_signal(&self) -> StopSignal {
    self.stop_signal.clone()
  }

  pub fn send_command(&self, message: Command) -> Option<ThreadResponse> {
    let envelope = CommandEnvelope {
      id: CommandId::next(),
      command: message,
    };
    let id = envelope.id;
    self.message_tx.try_send(envelope).ok()?;
    self.wake_server();
    self.pump_while_waiting_for_response(id)
  }

  pub fn next_event(&self) -> Option<Event> {
    self.acknowledge_last_event();

    if self.stop_signal.should_stop() {
      return None;
    }

    loop {
      if let Some(EventEnvelope { event, ack }) = self.event_backlog.pop() {
        if let Some(ack) = ack {
          self.pending_ack.lock().unwrap().replace(ack);
        }

        // if let Event::Window(WindowEvent::CloseRequest) = event {
        //   let x = self.shared.state_lock().close_on_x;
        //   if x {
        //     self.close();
        //   }
        // }

        return Some(event);
      }

      self.pump_event_and_block();
    }
  }

  fn wake_server(&self) {
    if let (Some(on_wake), Some(window)) = (self.on_wake.as_ref(), self.window.as_ref()) {
      on_wake(window);
    }
  }

  fn receive_event(&self) -> Option<ThreadEvent<ThreadResponse>> {
    let flow = *self.flow.lock().unwrap();
    match flow {
      Flow::Wait => self.event_rx.recv().ok(),
      Flow::Poll => match self.event_rx.try_recv() {
        Ok(event) => Some(event),
        Err(TryRecvError::Empty) => Some(ThreadEvent::Event(EventEnvelope::empty())),
        Err(TryRecvError::Disconnected) => None,
      },
    }
  }

  fn pump_event_and_block(&self) {
    match self.receive_event() {
      None => (),
      Some(ThreadEvent::Event(event)) => {
        self.event_backlog.push(event);
      },
      Some(ThreadEvent::CommandResponse(ResponseEnvelope { id, response })) => {
        self.responses.lock().unwrap().insert(id, response);
      },
    }
  }

  fn acknowledge_last_event(&self) {
    if let Some(token) = self.pending_ack.lock().unwrap().take() {
      token.send();
    }
  }

  fn pump_while_waiting_for_response(&self, id: CommandId) -> Option<ThreadResponse> {
    loop {
      if let Some(response) = self.responses.lock().ok()?.remove(&id) {
        return Some(response);
      }

      match self.event_rx.try_recv().ok()? {
        ThreadEvent::Event(EventEnvelope { event, ack }) => {
          self.event_backlog.push(EventEnvelope { event, ack: None });
          if let Some(ack) = ack {
            ack.send();
          }
        },
        ThreadEvent::CommandResponse(ResponseEnvelope { id, response }) => {
          self.responses.lock().ok()?.insert(id, response);
        },
      }
    }
  }
}
