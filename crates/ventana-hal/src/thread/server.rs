use {
  super::{
    command::CommandEnvelope,
    event::{
      EventEnvelope,
      // ThreadEvent,
    },
    response::ResponseEnvelope,
  },
  crate::{
    error::{
      MapToOSError,
      RequestError,
    },
    event::Event,
    thread::{
      response::Responses,
      signal::{
        AcknowledgeSignal,
        StopSignal,
      },
    },
  },
  crossbeam_channel::{
    Receiver,
    Sender,
  },
  std::sync::Arc,
};

pub struct ServerWrapper<Command, ThreadResponse>
where
  Self: Send + Sync + 'static,
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  server: Arc<dyn Server<Command = Command, ThreadResponse = ThreadResponse>>,
  event_tx: Sender<EventEnvelope>,
  message_rx: Receiver<CommandEnvelope<Command>>,

  responses: Responses<ThreadResponse>,

  stop_signal: StopSignal,
}

impl<Command, ThreadResponse> Drop for ServerWrapper<Command, ThreadResponse>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  fn drop(&mut self) {
    log::trace!("Dropping Server");
  }
}

impl<Command, ThreadResponse> ServerWrapper<Command, ThreadResponse>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  pub fn new(
    server: impl Server<Command = Command, ThreadResponse = ThreadResponse> + 'static,
    event_tx: Sender<EventEnvelope>,
    message_rx: Receiver<CommandEnvelope<Command>>,
    responses: Responses<ThreadResponse>,
    stop_signal: StopSignal,
  ) -> Result<Arc<Self>, RequestError> {
    let server = Arc::new(server);

    Ok(Arc::new(Self {
      server,
      event_tx,
      message_rx,
      responses,
      stop_signal,
    }))
  }

  pub fn run(self: Arc<Self>) -> Result<(), RequestError> {
    self.server.run(self.clone())?;
    Ok(())
  }

  // pub fn join(&self) -> Result<(), RequestError> {
  //   if let Some(thread) = self.handle.lock().unwrap().take() {
  //     log::trace!("Server thread joining main thread");
  //     thread.join().map_err(|_| os_error!("Failed to join main thread"))??;
  //     log::trace!("Server thread joined main thread");
  //   }
  //
  //   Ok(())
  // }

  // pub fn destroy(&self) {
  //   todo!()
  // }

  pub fn are_commands_pending(&self) -> bool {
    !self.message_rx.is_empty()
  }

  pub fn wait_for_command(&self) -> Result<CommandEnvelope<Command>, RequestError> {
    self.message_rx.recv().request_error()
  }

  pub fn receive_command(&self) -> Option<CommandEnvelope<Command>> {
    self.message_rx.try_recv().ok()
  }

  pub fn send_response(
    &self,
    response: ResponseEnvelope<ThreadResponse>,
    ack: Option<AcknowledgeSignal>,
  ) -> Result<(), RequestError> {
    log::trace!("Sending response `{:?}`", response.id);
    self.responses.insert(response);
    if let Some(ack) = ack {
      log::trace!("Acknowledging command");
      ack.heard();
    }
    // self
    //   .event_tx
    //   .try_send(ThreadEvent::CommandResponse(response))
    //   .map_err(|e| os_error_fmt!("failed to send response: `{e}`"))?; // try_send
    //
    Ok(())
  }

  pub fn send_event(&self, event: Event, should_block: bool) -> Result<(), RequestError> {
    if self.stop_signal.should_stop() {
      return Ok(()); // Gracefully ignore any events when stopping
    }

    log::trace!("Sending event `{event:?}` | should_block: {should_block}");
    let ack = should_block.then(AcknowledgeSignal::new);
    self
      .event_tx
      .try_send(EventEnvelope {
        event,
        ack: ack.clone(),
      })
      .map_err(|e| os_error_fmt!("failed to send event: `{e}`"))?; // try_send
    if let Some(ack) = ack {
      log::trace!("Waiting for event ack");
      ack.wait();
    }
    Ok(())
  }
}

pub trait Server: Send + Sync {
  type Command: Send + Sync;
  type ThreadResponse: Send + Sync;

  fn run(&self, server: Arc<ServerWrapper<Self::Command, Self::ThreadResponse>>) -> Result<(), RequestError>;
  // fn wake(&self) -> Result<(), RequestError>;
  // fn destroy(&self, client: &Client<Self::Command, Self::ThreadResponse>) -> Result<(), RequestError>;
}
