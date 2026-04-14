/*! TODO:
  Separate this out into a separate crate, and make it more generic.
*/

pub mod acknowledge;
pub mod client;
pub mod command;
pub mod event;
pub mod response;
pub mod server;
pub mod signal;

use {
  self::{
    client::Client,
    server::Server,
    signal::StopSignal,
  },
  crate::{
    error::{
      MapToOSError,
      RequestError,
    },
    event::Event,
    types::Flow,
  },
  std::sync::Arc,
};

pub struct ThreadLoop<Window, Command, ThreadResponse>
where
  Self: Send + Sync + 'static,
  Window: Send + Sync + 'static,
  Command: Send + Sync + 'static,
  ThreadResponse: Send + Sync + 'static,
{
  client: Client<Window, Command, ThreadResponse>,
  server: Arc<Server<Command, ThreadResponse>>,
  server_handle: Option<std::thread::JoinHandle<Result<(), RequestError>>>,
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

impl<Window: Send + Sync + 'static, Command: Send + Sync + 'static, ThreadResponse: Send + Sync + 'static>
  ThreadLoop<Window, Command, ThreadResponse>
{
  pub fn new(
    flow: Flow,
    on_drop: impl Fn(&Client<Window, Command, ThreadResponse>) + Send + Sync + 'static,
    on_wake: impl Fn(&Window) + Send + Sync + 'static,
  ) -> Self {
    let signal = StopSignal::new();
    let (event_tx, event_rx) = crossbeam_channel::unbounded();
    let (message_tx, message_rx) = crossbeam_channel::unbounded();

    let client = Client::new(signal.clone(), flow, event_rx, message_tx, on_drop, on_wake);
    let server = Arc::new(Server::new(event_tx, message_rx));

    Self {
      client,
      server,
      server_handle: None,
    }
  }

  pub fn run(
    &mut self,
    server_fn: impl FnOnce(Arc<Server<Command, ThreadResponse>>) -> Result<(), RequestError> + Send + 'static,
  ) -> Result<(), RequestError> {
    let server = self.server.clone();
    self.server_handle = Some(
      std::thread::Builder::new()
        .spawn(move || server_fn(server))
        .map_to_os_err()?,
    );
    Ok(())
  }

  pub fn loop_signal(&self) -> StopSignal {
    self.client.stop_signal()
  }

  pub fn set_window(&mut self, window: Window) {
    self.client.window = Some(window);
  }

  pub fn set_flow(&self, flow: Flow) {
    self.client.set_flow(flow);
  }

  pub fn send_command(&self, message: Command) -> Option<ThreadResponse> {
    self.client.send_command(message)
  }

  pub fn next_event(&self) -> Option<Event> {
    self.client.next_event()
  }

  pub fn join(&mut self) -> Result<(), RequestError> {
    self.client.destroy();

    let Some(thread) = self.server_handle.take() else {
      return Err(RequestError::Ignored);
    };

    log::trace!("Window thread joining main thread");
    thread.join().map_err(|_| os_error!("Failed to join main thread"))??;
    log::trace!("Window thread joined main thread");

    Ok(())
  }
}
