use std::ops::RangeInclusive;

use windows::Win32::{
  Foundation::{LPARAM, POINT, WPARAM},
  UI::WindowsAndMessaging::{DispatchMessageW, MSG, TranslateMessage},
};

use crate::{
  flag::PeekMessageFlags, get_message, handle::{window::WindowHandle, Win32Type}, peek_message, procedure::Response, GetMessageResult, PeekMessageResult
};

use super::{Message, data::MessageData, id::MessageId};

#[derive(Debug, Clone, PartialEq)]
pub struct Metadata {
  hwnd: WindowHandle,
  time: u32,
  pt: POINT,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ThreadMessage {
  message: Message,
  metadata: Metadata,
}

// make new() method for both With and Without and getters for the fields

impl From<MSG> for ThreadMessage {
  fn from(value: MSG) -> Self {
    Self {
      message: Message::from_raw(
        value.message.into(),
        MessageData {
          w: value.wParam.0,
          l: value.lParam.0,
        },
      ),
      metadata: Metadata {
        hwnd: value.hwnd.into(),
        time: value.time,
        pt: value.pt,
      },
    }
  }
}

impl From<ThreadMessage> for MSG {
  fn from(msg: ThreadMessage) -> Self {
    Self {
      hwnd: msg.metadata.hwnd.to_win32(),
      message: msg.message.id().to_u32(),
      wParam: WPARAM(msg.message.raw().w),
      lParam: LPARAM(msg.message.raw().l),
      time: msg.metadata.time,
      pt: msg.metadata.pt,
    }
  }
}

impl ThreadMessage {
  pub fn quit_requested(&self) -> bool {
    self.message.id() == MessageId::Destroy
  }

  pub fn get(hwnd: Option<WindowHandle>, filter: Option<RangeInclusive<u32>>) -> GetMessageResult {
    get_message(hwnd, &filter)
  }

  pub fn peek(
    hwnd: Option<WindowHandle>,
    filter: Option<RangeInclusive<u32>>,
    flags: PeekMessageFlags,
  ) -> PeekMessageResult {
    peek_message(hwnd, &filter, flags)
  }

  pub fn translate(&self) -> bool {
    let msg = MSG::from(self.clone());
    unsafe { TranslateMessage(&msg) }.as_bool()
  }

  pub fn dispatch(&self) -> Response {
    let msg = MSG::from(self.clone());
    Response(unsafe { DispatchMessageW(&msg) }.0)
  }

  pub fn window(&self) -> WindowHandle {
    self.metadata.hwnd
  }

  pub fn time(&self) -> u32 {
    self.metadata.time
  }

  pub fn point(&self) -> POINT {
    self.metadata.pt
  }
}
