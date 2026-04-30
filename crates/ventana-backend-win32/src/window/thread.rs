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
  widestring::WideCString,
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
      let mut class =
        WindowClass::builder().with_name("Window Class").with_style(WindowClassStyle::DoubleClicks);
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
  type Start = CreateInfo;

  fn start(&self, params: Self::Start, ctx: Arc<ThreadContext<Self>>) -> threadloop::Result<Self::Ready> {
    log::trace!("Creating window");

    let window = Self::create_window(ctx.clone(), params.shared.clone(), params.settings.clone())
      .map_err(|e| threadloop::Error::Other(e.to_string()))?;
    self.hwnd.lock().unwrap().replace(window);

    log::trace!("Sending window handle to main thread");

    Ok(window)
  }

  fn run(&self, _ctx: Arc<ThreadContext<Self>>) -> threadloop::Result<()> {
    log::trace!("Window is ready; entering message loop");

    MessageLoop::new().run();

    log::trace!("Joining main thread");

    Ok(())
  }

  fn send(&self, request: ClientToServer<Self::Request>) -> threadloop::Result<()> {
    if let Some(window) = self.hwnd.lock().unwrap().as_ref() {
      let msg = Message::App(AppMessage::new(Procedure::COMMAND_MESSAGE).with_data(request));
      window.post_message(msg).map_err(|e| {
        log::error!("{e}");
        threadloop::Error::OS(e.into())
      })?;
    }
    Ok(())
  }
}

#[allow(unused)]
pub struct Procedure {
  pub ctx: Arc<ThreadContext<Win32ThreadHandler>>,
  pub internal: Arc<SharedInternal>,
}

impl Procedure {
  pub const COMMAND_MESSAGE: u32 = Message::APP + 1;

  fn on_request(
    &self,
    window: &Window,
    request: ClientToServer<Command>,
  ) -> threadloop::Result<CommandResponse> {
    match request {
      ClientToServer::Request { request: Command::Redraw, .. } => {
        window.redraw().map_err(|e| threadloop::Error::OS(e.into()))?;
        Ok(CommandResponse::Success)
      },
      ClientToServer::Request { request: Command::GetWindowText, .. } => {
        let text = window.get_window_text().unwrap_or_default();
        Ok(CommandResponse::GetWindowText(text))
      },
      ClientToServer::Request { request: Command::SetWindowText(text), .. } => {
        window.set_window_text(text).map_err(|e| threadloop::Error::OS(e.into()))?;
        Ok(CommandResponse::Success)
      },
      ClientToServer::Stop { .. } => {
        let _ = window.destroy();
        Ok(CommandResponse::Success)
      },
      _ => Ok(CommandResponse::Success),
    }
  }
}

impl WindowProcedure for Procedure {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    // log::trace!("Received message: `{message:?}`");

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
        let _ = self.ctx.send_event(Event::Window(WindowEvent::CloseRequest));
        Some(LResult(0)) // We don't want defwindowproc to run since it'll auto-destroy the window
      },
      Message::SetText(SetTextMessage { l }) => {
        let text = unsafe { WideCString::from_ptr_str(l.0 as *const u16) };
        *self.internal.title.lock().unwrap() = text.to_string_lossy();
        None
      },
      Message::App(msg @ AppMessage { id: Self::COMMAND_MESSAGE, .. }) => {
        let request: ClientToServer<Command> = msg.clone().try_read().ok().unwrap();
        let _ = self.on_request(window, request).inspect_err(|e| log::error!("{e}"));
        None
      },
      Message::Destroy => {
        log::trace!("{window:?} | {message:?}");
        window.quit();
        None
      },
      _ => {
        if let Some(event) = map_native_event(message) {
          let _ = self.ctx.send_event(Event::Window(event));
        }
        None
      },
    }
  }
}
