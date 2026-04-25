use {
  super::signal::AcknowledgeSignal,
  crate::event::Event,
};

// #[derive(Debug)]
// pub enum ThreadEvent<ThreadResponse> {
//   // Ready(Result<ReadyInfo, RequestError>),
//   Event(EventEnvelope),
//   CommandResponse(ResponseEnvelope<ThreadResponse>),
// }

#[derive(Debug, Clone)]
pub struct EventEnvelope {
  pub event: Event,
  pub ack: Option<AcknowledgeSignal>,
}

impl EventEnvelope {
  pub fn empty() -> Self {
    Self {
      event: Event::None,
      ack: None,
    }
  }
}
