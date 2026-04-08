use {
  super::{
    command::Command,
    state::{
      Internal,
      State,
    },
    sync::SyncData,
  },
  crate::event::map_native_event,
  std::{
    sync::{
      Arc,
      MutexGuard,
      mpsc::SyncSender,
    },
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
    settings::WindowSettings,
    types::Stage,
  },
  win64::prelude::*,
};

pub struct WindowThread(Option<JoinHandle<Result<(), RequestError>>>);

impl WindowThread {
  pub fn new() -> Self {
    Self(None)
  }

  pub fn spawn(
    &mut self,
    window_sender: SyncSender<Window>,
    internal: Arc<Internal>,
    settings: WindowSettings,
  ) -> Result<(), RequestError> {
    if self.0.is_some() {
      return Err(RequestError::Ignored);
    }

    let handle = std::thread::Builder::new()
      .name("window".to_string())
      .spawn(move || Self::main(window_sender, internal, settings))
      .map_to_os_err()?;
    self.0 = Some(handle);
    Ok(())
  }

  pub fn join(&mut self) -> Result<(), RequestError> {
    let Some(thread) = self.0.take() else {
      return Err(RequestError::Ignored);
    };

    log::trace!("Window thread joining main thread");
    let _ = thread.join();
    log::trace!("Window thread joined window thread");

    Ok(())
  }

  fn main(
    window_sender: SyncSender<Window>,
    internal: Arc<Internal>,
    settings: WindowSettings,
  ) -> Result<(), RequestError> {
    log::trace!("Starting window thread");

    let window = Self::create_window(internal.clone(), settings)?;
    window_sender
      .send(window)
      .expect("Failed to send window back to main thread");

    internal.sync.next_frame.should_wait(true).unwrap();

    log::trace!("Entering message loop");

    MessageLoop::new().run();

    internal.state_lock().stage = Stage::Quit;
    internal.send_event_to_main(Event::LoopExiting);

    log::trace!("Joining main thread");
    Ok(())
  }

  fn create_window(internal: Arc<Internal>, settings: WindowSettings) -> Result<Window, RequestError> {
    let class = {
      let mut class = WindowClass::builder().with_name("Window Class");
      if let Some(color) = settings.clear_color {
        class = class.with_background_brush(Brush::solid(color));
      }
      class
    }
    .register()
    .map_to_os_err()?;
    let hwnd = class
      .create_window()
      .with_procedure(Procedure(internal))
      .with_name(settings.title.clone())
      .with_style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
      .with_position(settings.position)
      .with_size(Some(settings.size))
      .create()
      .map_to_os_err()?;
    Ok(hwnd)
  }
}

pub struct Procedure(Arc<Internal>);

impl Procedure {
  fn event_lock(&self) -> MutexGuard<'_, Option<Event>> {
    self.0.event_lock()
  }

  fn state_lock(&self) -> MutexGuard<'_, State> {
    self.0.state_lock()
  }

  fn sync(&self) -> &SyncData {
    &self.0.sync
  }
}

impl WindowProcedure for Procedure {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    let event = map_native_event(message);

    match (message, event) {
      (Message::Create(_), Some(WindowEvent::Created)) => {
        log::trace!("{window:?} | {message:?}");
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));

        self.event_lock().replace(Event::Window(WindowEvent::Created));
        self.sync().new_event.signal().unwrap();
      },
      (Message::SettingChange(_), _) => {
        log::trace!("{window:?} | {message:?}");
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
      },
      (Message::Close, _) => {
        log::trace!("{window:?} | {message:?}");
        self.0.send_event_to_main(Event::Window(WindowEvent::CloseRequest));
        return Some(LResult(0)); // We don't want defwindowproc to run since it'll auto-destroy the window
      },
      (Message::Destroy, _) => {
        log::trace!("{window:?} | {message:?}");
        self.0.send_event_to_main(Event::Window(WindowEvent::Destroyed));
        window.quit(); // SHOULD BE CHANGED
      },
      (
        Message::User(UserMessage {
          id: Command::MESSAGE_ID,
          w,
          ..
        }),
        _,
      ) => {
        log::trace!("{window:?} | {message:?}");
        let command = Command::from_raw(w.0);
        match *command {
          Command::Destroy => {
            log::trace!("Calling window.destroy()");
            window.destroy().unwrap();
          },
          Command::Redraw => {
            todo!()
          },
          // ...
          _ => (),
        }
      },
      (_, Some(event)) => {
        // log::trace!("{window:?} | {message:?} | {event:?}");
        self.0.send_event_to_main(Event::Window(event));
      },
      _ => (),
    }

    None
  }
}
