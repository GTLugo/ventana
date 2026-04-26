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
      ThreadContext,
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
    ctx: Arc<ThreadContext<Self>>,
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
    log::trace!("Creating window");

    let window = Self::create_window(ctx.clone(), params.shared.clone(), params.settings.clone())
      .map_err(|e| threadloop::Error::Other(e.to_string()))?;
    self.hwnd.lock().unwrap().replace(window);

    log::trace!("Sending window handle to main thread");

    ctx.signal_ready(Ok(window));

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
  pub ctx: Arc<ThreadContext<Win32ThreadHandler>>,
  pub internal: Arc<SharedInternal>,
}

impl Procedure {
  const DESTROY_MESSAGE: u32 = Message::APP + 11;

  fn handle_pending_commands(&mut self, window: &Window) -> Result<(), RequestError> {
    let mut handling_commands = true;
    while handling_commands {
      handling_commands = self
        .ctx
        .try_handle_request(|request| match request {
          ClientToServer::Request {
            request: Command::Redraw,
            ..
          } => {
            window.redraw().map_err(|e| threadloop::Error::OS(e.into()))?;
            Ok(CommandResponse::Success)
          },
          ClientToServer::Request {
            request: Command::GetWindowText,
            ..
          } => Ok(CommandResponse::GetWindowText(window.get_window_text().unwrap_or_default())),
          ClientToServer::Request {
            request: Command::SetWindowText(text),
            ..
          } => {
            window
              .set_window_text(text)
              .map_err(|e| threadloop::Error::OS(e.into()))?;
            Ok(CommandResponse::Success)
          },
          ClientToServer::Stop { .. } => {
            self.ctx.set_stopped();
            window
              .post_message(Message::App(AppMessage::empty(Self::DESTROY_MESSAGE)))
              .map_err(|e| threadloop::Error::OS(e.into()))?;
            Ok(CommandResponse::Success)
          },
          _ => Ok(CommandResponse::Success),
        })
        .request_error()?;
    }

    Ok(())
  }
}

impl WindowProcedure for Procedure {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    // log::trace!("Received message: `{message:?}`");

    if let Err(error) = self.handle_pending_commands(window) {
      log::error!("{error}");
    };

    // log::trace!("Handling message: `{message:?}`");

    match message {
      Message::Create(_) => {
        log::trace!("{window:?} | {message:?}");
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      },
      Message::SettingChange(_) => {
        log::trace!("{window:?} | {message:?}");
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
        None
      },
      Message::Close => {
        log::trace!("{window:?} | {message:?}");
        let _ = self.ctx.send_event(Event::Window(WindowEvent::CloseRequest));
        Some(LResult(0)) // We don't want defwindowproc to run since it'll auto-destroy the window
      },
      Message::App(AppMessage {
        id: Self::DESTROY_MESSAGE,
        ..
      }) => {
        let _ = window.destroy();
        None
      },
      Message::Destroy => {
        log::trace!("{window:?} | {message:?}");
        window.quit();
        None
      },
      _ => {
        if let Some(event) = map_native_event(message) {
          // log::trace!("{window:?} | {message:?} | {event:?}");
          let _ = self.ctx.send_event(Event::Window(event));
        }
        None
      },
    }
  }
}
