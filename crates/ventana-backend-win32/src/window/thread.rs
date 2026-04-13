use {
  super::{
    command::Command,
    state::SharedInternal,
  },
  crate::{
    event::map_native_event,
    window::{
      command::{
        CommandEnvelope,
        CommandResponse,
      },
      sync::ResponseEnvelope,
    },
  },
  crossbeam_channel::Sender,
  std::{
    sync::Arc,
    thread::JoinHandle,
  },
  ventana_hal::{
    error::{
      MapToOSError,
      RequestError,
    },
    event::{
      Event,
      WindowEvent,
    },
    os_error,
    settings::WindowSettings,
  },
  win64::prelude::*,
};

pub struct WindowThread {
  handle: Option<JoinHandle<Result<(), RequestError>>>,
}

impl Drop for WindowThread {
  fn drop(&mut self) {
    if self.handle.is_some() {
      // Make absolute sure the thread joins before dropping this object
      self.join().expect("panicked when attempting to join Window thread");
    }
  }
}

impl WindowThread {
  pub fn new() -> Self {
    Self { handle: None }
  }

  pub fn spawn(&mut self, internal: Arc<SharedInternal>, settings: WindowSettings) -> Result<Window, RequestError> {
    if self.handle.is_some() {
      return Err(RequestError::Ignored);
    }

    let (window_tx, window_rx) = crossbeam_channel::bounded(0);

    let handle = std::thread::Builder::new()
      .name("window".to_string())
      .spawn(move || Self::main(window_tx, internal, settings))
      .map_to_os_err()?;
    self.handle = Some(handle);

    log::trace!("Waiting to receive window handle back from window thread");

    let hwnd = window_rx
      .recv()
      .expect("Failed to receive window back from window thread");

    log::trace!("Received window handle from window thread: `{hwnd:?}`");

    Ok(hwnd)
  }

  pub fn join(&mut self) -> Result<(), RequestError> {
    let Some(thread) = self.handle.take() else {
      return Err(RequestError::Ignored);
    };

    log::trace!("Window thread joining main thread");
    thread.join().map_err(|_| os_error!("Failed to join main thread"))??;
    log::trace!("Window thread joined main thread");

    Ok(())
  }

  fn main(
    window_tx: Sender<Window>,
    internal: Arc<SharedInternal>,
    settings: WindowSettings,
  ) -> Result<(), RequestError> {
    log::trace!("Starting window thread");

    let window = Self::create_window(internal.clone(), settings)?;
    window_tx
      .send(window)
      .expect("Failed to send window back to main thread");
    internal.set_ready();

    log::trace!("Entering message loop");

    MessageLoop::new().run();

    log::trace!("Joining main thread");
    Ok(())
  }

  fn create_window(internal: Arc<SharedInternal>, settings: WindowSettings) -> Result<Window, RequestError> {
    let class = {
      let mut class = WindowClass::builder()
        .with_name("Window Class")
        .with_style(WindowClassStyle::DoubleClicks);
      if let Some(color) = settings.clear_color {
        class = class.with_background_brush(Brush::solid(color));
      }
      class
    }
    .register()
    .map_to_os_err()?;
    log::debug!("{settings:?}");
    let hwnd = class
      .create_window()
      .with_procedure(Procedure { internal })
      .with_name(settings.title.clone())
      .with_style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
      .with_position(settings.position)
      .with_size(Some(settings.size))
      .create()
      .map_to_os_err()?;
    Ok(hwnd)
  }
}

pub struct Procedure {
  internal: Arc<SharedInternal>,
}

impl Procedure {
  fn send_event(&self, event: Event) {
    self.internal.send_event(event, self.internal.is_ready());
  }

  fn receive_command(&self) -> Option<CommandEnvelope> {
    self.internal.receive_command()
  }
}

impl WindowProcedure for Procedure {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    // log::trace!("Received message: `{:?}`", message);

    let event = map_native_event(message);

    if self.internal.are_commands_pending() {
      // log::trace!("Commands are pending, processing them before handling the message");
      while let Some(CommandEnvelope { id, command }) = self.receive_command() {
        log::trace!("Received command in window procedure: `{command:?}`");
        match command {
          Command::Destroy => {
            window.destroy().unwrap();
            self.internal.send_response(ResponseEnvelope {
              id,
              response: CommandResponse::Empty,
            });
          },
          Command::Redraw => {
            window.redraw().unwrap();
            self.internal.send_response(ResponseEnvelope {
              id,
              response: CommandResponse::Empty,
            });
          },
          Command::GetWindowText => {
            self.internal.send_response(ResponseEnvelope {
              id,
              response: CommandResponse::GetWindowText(window.get_window_text().unwrap_or_default()),
            });
          },
          _ => (),
        }
      }
    }

    match (message, event) {
      (Message::Create(_), _) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      },
      (Message::SettingChange(_), _) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      },
      (Message::Close, _) => {
        self.send_event(Event::Window(WindowEvent::CloseRequest));
        Some(LResult(0)) // We don't want defwindowproc to run since it'll auto-destroy the window
      },
      (Message::Destroy, _) => {
        window.quit();
        None
      },
      (_, Some(event)) => {
        // log::trace!("{window:?} | {message:?} | {event:?}");
        self.send_event(Event::Window(event));
        None
      },
      _ => None,
    }
  }
}
