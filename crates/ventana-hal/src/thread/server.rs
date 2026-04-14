use {
  super::{
    acknowledge::AcknowledgementToken,
    command::CommandEnvelope,
    event::{
      EventEnvelope,
      ThreadEvent,
    },
    response::ResponseEnvelope,
  },
  crate::{
    error::RequestError,
    event::Event,
  },
  crossbeam_channel::{
    Receiver,
    Sender,
  },
};

pub struct Server<Command, ThreadResponse>
where
  Self: Send + Sync + 'static,
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  event_tx: Sender<ThreadEvent<ThreadResponse>>,
  message_rx: Receiver<CommandEnvelope<Command>>,
}

impl<Command: Send + Sync + 'static, ThreadResponse: Send + Sync + 'static> Server<Command, ThreadResponse> {
  pub fn new(event_tx: Sender<ThreadEvent<ThreadResponse>>, message_rx: Receiver<CommandEnvelope<Command>>) -> Self {
    Self { event_tx, message_rx }
  }

  pub fn are_commands_pending(&self) -> bool {
    !self.message_rx.is_empty()
  }

  pub fn receive_command(&self) -> Option<CommandEnvelope<Command>> {
    self.message_rx.try_recv().ok()
  }

  pub fn send_response(&self, response: ResponseEnvelope<ThreadResponse>) -> Result<(), RequestError> {
    self
      .event_tx
      .try_send(ThreadEvent::CommandResponse(response))
      .map_err(|e| os_error_fmt!("failed to send response: `{e}`"))?;
    Ok(())
  }

  pub fn send_event(&self, event: Event, should_block: bool) -> Result<(), RequestError> {
    let (ack, receiver) = if should_block {
      let (tx, rx) = AcknowledgementToken::new();
      (Some(tx), Some(rx))
    } else {
      (None, None)
    };
    self
      .event_tx
      .try_send(ThreadEvent::Event(EventEnvelope { event, ack }))
      .map_err(|e| os_error_fmt!("failed to send event: `{e}`"))?;
    if let Some(receiver) = receiver {
      receiver
        .recv()
        .map_err(|e| os_error_fmt!("failed to receive acknowledgement: `{e}`"))?;
    }
    Ok(())
  }
}
