use {
  crate::window::command::{
    CommandId,
    CommandResponse,
  },
  crossbeam_channel::{
    Receiver,
    Sender,
  },
  ventana_hal::event::Event,
};

#[derive(Debug, Clone)]
pub struct AcknowledgementToken(Sender<()>);

impl AcknowledgementToken {
  pub fn new() -> (Self, Receiver<()>) {
    let (ack_tx, ack_rx) = crossbeam_channel::bounded(0);
    (Self(ack_tx), ack_rx)
  }

  pub fn send(self) {
    self.0.send(()).unwrap();
  }
}

#[derive(Debug, Clone)]
pub struct EventEnvelope {
  pub event: Event,
  pub ack: Option<AcknowledgementToken>,
}

impl EventEnvelope {
  pub fn empty() -> Self {
    Self {
      event: Event::None,
      ack: None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct ResponseEnvelope {
  pub id: CommandId,
  pub response: CommandResponse,
}

#[derive(Debug)]
pub enum WindowToMain {
  // Ready(Result<ReadyInfo, RequestError>),
  Event(EventEnvelope),
  CommandResponse(ResponseEnvelope),
}
