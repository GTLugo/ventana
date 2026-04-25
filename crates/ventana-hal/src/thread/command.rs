// #[derive(Debug)]
// pub enum ThreadCommand<Command> {
//   Command(CommandEnvelope<Command>),
//   Stop,
// }

use {
  crate::thread::{
    response::ResponseEnvelope,
    signal::AcknowledgeSignal,
  },
  std::sync::atomic::{
    AtomicU64,
    Ordering,
  },
};

#[derive(Debug, Clone)]
pub struct CommandEnvelope<Command> {
  pub id: CommandId,
  pub command: Command,
  pub ack: Option<AcknowledgeSignal>,
}

impl<Command> CommandEnvelope<Command> {
  pub fn new(command: Command) -> Self {
    Self {
      id: CommandId::next(),
      command,
      ack: Some(AcknowledgeSignal::new()),
    }
  }
}

impl<Command> From<Command> for CommandEnvelope<Command> {
  fn from(command: Command) -> Self {
    Self::new(command)
  }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct CommandId(u64);

impl CommandId {
  fn next() -> Self {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    Self(COUNTER.fetch_add(1, Ordering::Relaxed))
  }

  pub fn new_response<ThreadResponse>(self, response: ThreadResponse) -> ResponseEnvelope<ThreadResponse> {
    ResponseEnvelope { response, id: self }
  }
}
