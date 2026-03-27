use {
  cursor_icon::CursorIcon,
  ventana_hal::{
    dpi::{
      Position,
      Size,
    },
    types::{
      CursorMode,
      Fullscreen,
      Visibility,
    },
  },
  win64::user::{
    Message,
    UserMessage,
    WParam,
    Window,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub enum Command {
  Close,
  Destroy,
  Redraw,
  SetVisibility(Visibility),
  SetDecorations(Visibility),
  SetWindowText(String),
  SetSize(Size),
  SetPosition(Position),
  SetFullscreen(Option<Fullscreen>),
  SetCursorIcon(CursorIcon),
  SetCursorMode(CursorMode),
  SetCursorVisibility(Visibility),
}

impl Command {
  pub const MESSAGE_ID: u32 = win64::sys::WM_USER + 69;

  pub fn from_raw(raw: usize) -> Box<Self> {
    unsafe { Box::from_raw(raw as *mut Command) }
  }

  pub fn post(self, window: Window) {
    let command = Box::leak(Box::new(self));
    let addr = command as *mut Command as usize;
    window
      .post_message(Message::User(UserMessage {
        id: Self::MESSAGE_ID,
        w: WParam(addr),
        ..Default::default()
      }))
      .unwrap();
  }

  pub fn send(self, window: Window) {
    let command = Box::leak(Box::new(self));
    let addr = command as *mut Command as usize;
    window.send_message(Message::User(UserMessage {
      id: Self::MESSAGE_ID,
      w: WParam(addr),
      ..Default::default()
    }));
  }
}
