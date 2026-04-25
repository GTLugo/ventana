use {
  crate::{
    event::Event,
    thread::{
      command::{
        CommandEnvelope,
        CommandId,
      },
      event::EventEnvelope,
      response::ResponseEnvelope,
      signal::AcknowledgeSignal,
    },
  },
  crossbeam_queue::SegQueue,
  std::{
    collections::HashMap,
    sync::Mutex,
  },
};

#[derive(Debug)]
pub struct Queues<Command, ThreadResponse>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  events: SegQueue<EventEnvelope>,
  commands: SegQueue<CommandEnvelope<Command>>,
  responses: Mutex<HashMap<CommandId, ThreadResponse>>,
}

impl<Command, ThreadResponse> Default for Queues<Command, ThreadResponse>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  fn default() -> Self {
    Self {
      events: Default::default(),
      commands: Default::default(),
      responses: Default::default(),
    }
  }
}

impl<Command, ThreadResponse> Queues<Command, ThreadResponse>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  pub fn new() -> Self {
    Default::default()
  }

  pub fn are_events_pending(&self) -> bool {
    !self.events.is_empty()
  }

  pub fn are_commands_pending(&self) -> bool {
    !self.commands.is_empty()
  }

  pub fn pop_event(&self) -> Option<EventEnvelope> {
    self.events.pop()
  }

  pub fn pop_command(&self) -> Option<CommandEnvelope<Command>> {
    self.commands.pop()
  }

  pub fn get_response(&self, id: CommandId) -> Option<ThreadResponse> {
    self.responses.lock().unwrap().remove(&id)
  }

  pub fn push_event(&self, event: Event, should_block: bool) {
    // log::trace!("Pushing event `{event:?}` | should_block: {should_block}");
    let ack = should_block.then(AcknowledgeSignal::new);
    self.events.push(EventEnvelope {
      event,
      ack: ack.clone(),
    });
    if let Some(ack) = ack {
      ack.wait();
    }
  }

  pub fn push_command(&self, message: Command) -> CommandId {
    let envelope = CommandEnvelope::new(message);
    let id = envelope.id;
    // log::trace!("Pushing command `{id:?}`");
    self.commands.push(envelope);
    id
  }

  pub fn insert_response(&self, response: ResponseEnvelope<ThreadResponse>) {
    self.responses.lock().unwrap().insert(response.id, response.response);
  }
}
