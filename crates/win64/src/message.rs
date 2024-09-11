use std::ops::RangeInclusive;

use windows::Win32::{
  Foundation::{self, LPARAM, WPARAM},
  UI::WindowsAndMessaging::{self, DispatchMessageW, TranslateMessage, MSG},
};

use crate::{
  flag::PeekMessageFlags,
  get_message,
  handle::{window::WindowId, Win32Type},
  peek_message, GetMessageResult, PeekMessageResult, ProcedureResult,
};

use self::input::KeyboardMessage;

pub mod input;
pub mod pump;

// pub enum PumpType {
//   Wait,
//   Poll,
// }

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
  hwnd: WindowId,
  time: u32,
  pt: Foundation::POINT,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoMetadata;

#[derive(Debug, Clone, PartialEq)]
pub struct RawMessage<M = NoMetadata> {
  id: u32,
  w: usize,
  l: isize,
  metadata: M,
}

// make new() method for both With and Without and getters for the fields

impl From<MSG> for RawMessage<Metadata> {
  fn from(value: MSG) -> Self {
    Self {
      id: value.message,
      w: value.wParam.0,
      l: value.lParam.0,
      metadata: Metadata {
        hwnd: value.hwnd.into(),
        time: value.time,
        pt: value.pt,
      },
    }
  }
}

impl From<RawMessage<Metadata>> for MSG {
  fn from(msg: RawMessage<Metadata>) -> Self {
    Self {
      hwnd: msg.window().to_win32(),
      message: msg.id(),
      wParam: WPARAM(msg.w()),
      lParam: LPARAM(msg.l()),
      time: msg.time(),
      pt: *msg.point(),
    }
  }
}

impl<M> RawMessage<M> {
  pub const QUIT: u32 = WindowsAndMessaging::WM_QUIT;
  pub const DESTROY: u32 = WindowsAndMessaging::WM_DESTROY;

  pub fn id(&self) -> u32 {
    self.id
  }

  pub fn w(&self) -> usize {
    self.w
  }

  pub fn l(&self) -> isize {
    self.l
  }

  pub fn quit_requested(&self) -> bool {
    self.id == Self::DESTROY
  }
}

impl RawMessage<NoMetadata> {
  pub fn new(msg: u32, w: WPARAM, l: LPARAM) -> Self {
    Self {
      id: msg,
      w: w.0,
      l: l.0,
      metadata: NoMetadata,
    }
  }

  pub fn parse<F: FromMessage>(&self) -> Result<F, F::Err> {
    FromMessage::from_message(self)
  }
}

impl RawMessage<Metadata> {
  pub fn get(hwnd: Option<WindowId>, filter: Option<RangeInclusive<u32>>) -> GetMessageResult {
    get_message(hwnd, &filter)
  }

  pub fn peek(
    hwnd: Option<WindowId>,
    filter: Option<RangeInclusive<u32>>,
    flags: PeekMessageFlags,
  ) -> PeekMessageResult {
    peek_message(hwnd, &filter, flags)
  }

  pub fn translate(&self) -> bool {
    let msg = MSG::from(self.clone());
    unsafe { TranslateMessage(&msg) }.as_bool()
  }

  pub fn dispatch(&self) -> ProcedureResult {
    let msg = MSG::from(self.clone());
    unsafe { DispatchMessageW(&msg) }.into()
  }

  pub fn window(&self) -> &WindowId {
    &self.metadata.hwnd
  }

  pub fn time(&self) -> u32 {
    self.metadata.time
  }

  pub fn point(&self) -> &Foundation::POINT {
    &self.metadata.pt
  }
}

pub trait FromMessage: Sized {
  type Err;

  fn from_message(msg: &RawMessage) -> Result<Self, Self::Err>;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
  Other(RawMessage),
  CloseRequested,
  Keyboard { message: KeyboardMessage, raw: RawMessage },
}

impl Message {
  pub fn new(msg: u32, w: WPARAM, l: LPARAM) -> Self {
    RawMessage::new(msg, w, l).into()
  }

  pub fn id(&self) -> u32 {
    match self {
      Message::Other(msg) => msg.id,
      Message::CloseRequested => WindowsAndMessaging::WM_CLOSE,
      Message::Keyboard { raw, .. } => raw.id(),
    }
  }

  pub fn w(&self) -> usize {
    match self {
      Message::Other(msg) => msg.w,
      Message::CloseRequested => 0,
      Message::Keyboard { raw, .. } => raw.w(),
    }
  }

  pub fn l(&self) -> isize {
    match self {
      Message::Other(msg) => msg.l,
      Message::CloseRequested => 0,
      Message::Keyboard { raw, .. } => raw.l(),
    }
  }
}

impl From<RawMessage> for Message {
  fn from(value: RawMessage) -> Self {
    match value.id() {
      WindowsAndMessaging::WM_CLOSE => Self::CloseRequested,
      WindowsAndMessaging::WM_KEYFIRST..=WindowsAndMessaging::WM_KEYLAST => Self::Keyboard {
        message: value.parse::<KeyboardMessage>().unwrap(),
        raw: value,
      },
      _ => Self::Other(value),
    }
  }
}
