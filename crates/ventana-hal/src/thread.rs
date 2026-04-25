/*! TODO:
  Separate this out into a separate crate, and make it more generic.
*/

pub mod client;
pub mod command;
pub mod event;
pub mod queues;
pub mod response;
pub mod server;
pub mod signal;

use {
  self::{
    client::Client,
    server::ServerWrapper,
    signal::StopSignal,
  },
  crate::{
    error::{
      MapToOSError,
      RequestError,
    },
    event::Event,
    thread::{
      client::ClientWrapper,
      response::Responses,
      server::Server,
    },
    types::Flow,
  },
  std::sync::{
    Arc,
    Mutex,
  },
};

pub struct ThreadLoop<Command, ThreadResponse, C>
where
  Self: Send + Sync + 'static,
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
  C: Client<Command = Command, ThreadResponse = ThreadResponse>,
{
  is_running: bool,
  client: ClientWrapper<Command, ThreadResponse, C>,
  server: Arc<ServerWrapper<Command, ThreadResponse>>,
  handle: Mutex<Option<std::thread::JoinHandle<Result<(), RequestError>>>>,
}

impl<Command, ThreadResponse, C> Drop for ThreadLoop<Command, ThreadResponse, C>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
  C: Client<Command = Command, ThreadResponse = ThreadResponse>,
{
  fn drop(&mut self) {
    log::trace!("Dropping ThreadLoop");
  }
}

// impl<Command: Send + Sync + 'static, ThreadResponse: Send + Sync + 'static> Drop
//   for ThreadLoop<Command, ThreadResponse>
// {
//   fn drop(&mut self) {
//     log::trace!("Dropping ThreadLoop");
//     if self.server_handle.is_some() {
//       // Make absolute sure the thread joins before dropping this object
//       self.join().expect("panicked when attempting to join Window thread");
//     }
//   }
// }

// impl<Command, ThreadResponse> Default for ThreadLoop<Command, ThreadResponse> {
//   fn default() -> Self {
//     Self::new(Flow::Wait, || {})
//   }
// }

impl<Command, ThreadResponse, C> ThreadLoop<Command, ThreadResponse, C>
where
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
  C: Client<Command = Command, ThreadResponse = ThreadResponse>,
{
  pub fn new(
    flow: Flow,
    client: C,
    server: impl Server<Command = Command, ThreadResponse = ThreadResponse> + 'static,
  ) -> Result<Self, RequestError> {
    let signal = StopSignal::new();
    let (event_tx, event_rx) = crossbeam_channel::unbounded();
    let (message_tx, message_rx) = crossbeam_channel::unbounded();
    let responses = Responses::new();

    let client = ClientWrapper::new(client, signal.clone(), flow, event_rx, message_tx, responses.clone());
    let server = ServerWrapper::new(server, event_tx, message_rx, responses, signal)?;

    Ok(Self {
      is_running: false,
      client,
      server,
      handle: Mutex::new(None),
    })
  }

  pub fn run(&mut self) -> Result<(), RequestError> {
    if self.is_running {
      return Err(RequestError::Ignored);
    }
    self.is_running = true;

    let server = self.server.clone();

    *self.handle.lock().unwrap() = Some(
      std::thread::Builder::new()
        .name("window".to_string())
        .spawn(move || server.run())
        .map_to_os_err()?,
    );

    Ok(())
  }

  pub fn client(&self) -> &C {
    self.client.client()
  }

  pub fn client_mut(&mut self) -> &mut C {
    self.client.client_mut()
  }

  pub fn stop_signal(&self) -> StopSignal {
    self.client.stop_signal()
  }

  pub fn set_flow(&self, flow: Flow) {
    self.client.set_flow(flow);
  }

  pub fn send_request(&self, message: Command, wake_server: bool) -> Option<ThreadResponse> {
    self.client.send_request(message, wake_server)
  }

  pub fn send_command(&self, message: Command, wake_server: bool) {
    self.client.send_command(message, wake_server)
  }

  pub fn next_event(&self) -> Option<Event> {
    self.client.next_event()
  }

  pub fn join(&mut self) -> Result<(), RequestError> {
    self.client.destroy();

    // self.server.join()?;
    if let Some(thread) = self.handle.lock().unwrap().take() {
      log::trace!("Waiting for server thread to join main thread");
      thread
        .join()
        .map_err(|_| os_error!("Server thread failed to join main thread"))??;
      log::trace!("Server thread joined main thread");
    }

    Ok(())
  }
}
