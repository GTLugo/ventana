use {
  super::{
    create_info::CreateInfo,
  },
  std::{
    sync::mpsc::SyncSender,
    thread::JoinHandle,
  }, ventana_hal::error::{MapToOSError, RequestError}, win64::prelude::*,
};

pub struct WindowThread(Option<JoinHandle<Result<(), RequestError>>>);

impl WindowThread {
  pub fn spawn(window_sender: SyncSender<Window>, create_info: CreateInfo) -> Result<Self, RequestError> {
    let handle = std::thread::Builder::new()
        .name("window".to_string())
        .spawn(move || Self::main(window_sender, create_info))
        .map_to_os_err()?;
    Ok(Self(Some(handle)))
  }

  fn main(window_sender: SyncSender<Window>, create_info: CreateInfo) -> Result<(), RequestError> {
    log::trace!("Starting window thread");

    let window = Self::create_window(create_info)?;
    window_sender.send(window).expect("Failed to send window back to main thread");

    log::trace!("Entering message loop");

    for message in Msg::get(MsgQueue::CurrentThread, None).flatten() {
      message.translate();
      message.dispatch();
    }

    log::trace!("Joining main thread");
    Ok(())
  }

  fn create_window(create_info: CreateInfo) -> Result<Window, RequestError> {
    let class = WindowClass::builder().name("Window Class").register().map_to_os_err()?;
    let hwnd = class
      .window_builder()
      .procedure(Internal)
      .name(create_info.settings.title.clone())
      .style(WindowStyle::OverlappedWindow | WindowStyle::Visible)
      .position(create_info.settings.position)
      .size(Some(create_info.settings.size))
      .create()
      .map_to_os_err()?;
    Ok(hwnd)
  }
}

struct Internal;

impl WindowProcedure for Internal {
  fn on_message(&mut self, window: &Window, message: &Message) -> Option<LResult> {
    log::trace!("{window:?} | {message:?}");
    match message {
      Message::Create(_) | Message::SettingChange(_) => {
        window.dwm_set_window_attribute(DwmWindowAttribute::UseImmersiveDarkMode(is_os_dark_mode()));
      },
      Message::Destroy => {
        window.quit(); // SHOULD BE CHANGED
      },
      _ => (),
    }

    None
  }
}
