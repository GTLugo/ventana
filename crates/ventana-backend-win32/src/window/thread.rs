use {
  super::{
    command::{
      Command,
      CreateInfo,
    },
    state::SharedInternal,
  },
  crate::{
    event::map_native_event,
    window::command::CommandResponse,
  },
  std::sync::{
    Arc,
    Mutex,
  },
  threadloop::{
    context::{
      Context,
      ThreadHandler,
    },
    message::ClientToServer,
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
    settings::WindowSettings,
  },
  win64::prelude::*,
};

pub struct Win32ThreadHandler {
  hwnd: Mutex<Option<Window>>,
}

impl Win32ThreadHandler {
  pub fn new() -> Self {
    Self { hwnd: Mutex::new(None) }
  }
}

impl Win32ThreadHandler {
  fn create_window(
    ctx: Arc<Context<Event, Command, CommandResponse, CreateInfo, Window>>,
    internal: Arc<SharedInternal>,
    settings: WindowSettings,
  ) -> Result<Window, RequestError> {
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
      .with_procedure(Procedure { ctx, internal })
      .with_name(settings.title.clone())
      .with_style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
      .with_position(settings.position)
      .with_size(Some(settings.size))
      .create()
      .map_to_os_err()?;
    Ok(hwnd)
  }
}

impl ThreadHandler for Win32ThreadHandler {
  type Event = Event;
  type Ready = Window;
  type Request = Command;
  type Response = CommandResponse;
  type Start = CreateInfo;

  fn run(
    &self,
    params: Self::Start,
    ctx: Arc<Context<Self::Event, Self::Request, Self::Response, Self::Start, Self::Ready>>,
  ) -> threadloop::Result<()> {
    log::trace!("Starting window thread");

    // log::trace!("Waiting for Command::CreateWindow...");

    // // REPLACE WAIT_FOR WITH A BARRIER / WAIT GROUP
    // let CommandEnvelope {
    //   id,
    //   command: Command::CreateWindow(create_info),
    //   ack
    // } = ctx.wait_for_command()?
    // else {
    //   unreachable!("First command should always be CreateWindow");
    // };

    log::trace!("Creating window");

    let window = Self::create_window(ctx.clone(), params.shared.clone(), params.settings.clone())
      .map_err(|e| threadloop::Error::Other(e.to_string()))?;
    self.hwnd.lock().unwrap().replace(window);

    log::trace!("Sending window handle to main thread");

    ctx.signal_ready(Ok(window));
    params.shared.set_ready(true);

    // server_thread.send_response(id.new_response(CommandResponse::CreateWindow(window)), ack)?;

    log::trace!("Window is ready; entering message loop");

    MessageLoop::new().run();

    log::trace!("Joining main thread");

    Ok(())
  }

  fn wake(&self) -> threadloop::Result<()> {
    const WAKE_MESSAGE: u32 = Message::APP + 67;
    if let Some(window) = self.hwnd.lock().unwrap().as_ref() {
      window
        .post_message(Message::App(AppMessage::empty(WAKE_MESSAGE)))
        .map_err(|e| threadloop::Error::OS(e.into()))?;
    }
    Ok(())
  }
}

pub struct Procedure {
  pub ctx: Arc<Context<Event, Command, CommandResponse, CreateInfo, Window>>,
  pub internal: Arc<SharedInternal>,
}

impl Procedure {
  // fn send_event(&self, event: Event) {
  //   self.ctx.send_event(event, self.internal.is_ready()).unwrap();
  // }

  // fn on_command(&mut self, window: &Window, CommandEnvelope { id, command, ack }: CommandEnvelope<Command>) {
  //   log::trace!("Handling command: `{command:?}`");
  //   match command {
  //     Command::Destroy => {
  //       window.destroy().unwrap();
  //       self
  //         .ctx
  //         .send_response(id.new_response(CommandResponse::Success), ack)
  //         .unwrap();
  //     },
  //     Command::Redraw => {
  //       window.redraw().unwrap();
  //       self
  //         .server
  //         .send_response(id.new_response(CommandResponse::Success), ack)
  //         .unwrap();
  //     },
  //     Command::GetWindowText => {
  //       self
  //         .server
  //         .send_response(
  //           id.new_response(CommandResponse::GetWindowText(window.get_window_text().unwrap_or_default())),
  //           ack,
  //         )
  //         .unwrap();
  //     },
  //     Command::SetWindowText(text) => {
  //       window.set_window_text(&text).unwrap();
  //       self
  //         .server
  //         .send_response(id.new_response(CommandResponse::Success), ack)
  //         .unwrap();
  //     },
  //     _ => (),
  //   }
  // }
}

impl WindowProcedure for Procedure {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    log::trace!("Received message: `{message:?}`");

    // if self.server.are_commands_pending() {
    // log::trace!("Commands are pending, processing them before handling the message");
    let mut handling_commands = true;
    // while let Some(command_envelope) = self.ctx.receive_command() {
    //   self.on_command(window, command_envelope);
    // }
    while handling_commands {
      handling_commands = self
        .ctx
        .try_handle_request(|request| {
          match request {
            // ClientToServer::Request {
            //   request: Command::Destroy,
            //   ..
            // } => {
            //   window.destroy();
            //   Ok(CommandResponse::Success)
            // },
            ClientToServer::Request {
              request: Command::Redraw,
              ..
            } => {
              window.redraw();
              Ok(CommandResponse::Success)
            },
            ClientToServer::Request {
              request: Command::GetWindowText,
              ..
            } => {
              window.destroy();
              Ok(CommandResponse::GetWindowText(window.get_window_text().unwrap_or_default()))
            },
            ClientToServer::Request {
              request: Command::SetWindowText(text),
              ..
            } => {
              window.set_window_text(text);
              Ok(CommandResponse::Success)
            },
            ClientToServer::Stop { .. } => {
              window.destroy();
              Ok(CommandResponse::Success)
            },
            _ => Ok(CommandResponse::Success),
          }
        })
        .unwrap();
    }
    // }

    log::trace!("Handling message: `{message:?}`");

    match message {
      Message::Create(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      },
      Message::SettingChange(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      },
      Message::Close => {
        self.ctx.send_event(Event::Window(WindowEvent::CloseRequest));
        Some(LResult(0)) // We don't want defwindowproc to run since it'll auto-destroy the window
      },
      Message::Destroy => {
        window.quit();
        None
      },
      _ => {
        if let Some(event) = map_native_event(message) {
          // log::trace!("{window:?} | {message:?} | {event:?}");
          self.ctx.send_event(Event::Window(event));
        }
        None
      },
    }
  }
}
