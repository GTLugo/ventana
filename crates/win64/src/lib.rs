pub mod class;
pub mod descriptor;
pub mod flag;
pub mod handle;
pub mod message;
pub mod procedure;
pub mod types;

pub use dpi;

use std::{io, ops::RangeInclusive};

use self::{
  flag::PeekMessageFlags,
  handle::{Win32Type, window::WindowHandle},
  message::thread::ThreadMessage,
};
use thiserror::Error;
use windows::Win32::Foundation::ERROR_SUCCESS;
use windows::Win32::UI::WindowsAndMessaging::{self, GetMessageW, MSG, PeekMessageW};

#[derive(Error, Debug)]
pub enum Error {
  #[error("{0}")]
  IOError(#[from] io::Error),
  #[error("{0}")]
  Win32Error(#[from] windows::core::Error),
}

#[derive(Debug)]
pub enum GetMessageResult {
  Message(ThreadMessage),
  Quit,
  Error(windows::core::Error),
}

#[derive(Debug)]
pub enum PeekMessageResult {
  Message(ThreadMessage),
  Quit,
  None,
}

fn get_last_error() -> Option<Error> {
  let error = windows::core::Error::from_win32();
  if error.code() == ERROR_SUCCESS.to_hresult() {
    None
  } else {
    Some(Error::Win32Error(error))
  }
}

fn get_message(hwnd: Option<WindowHandle>, filter: &Option<RangeInclusive<u32>>) -> GetMessageResult {
  let (min, max) = match filter {
    Some(filter) => (*filter.start(), *filter.end()),
    None => (0, 0),
  };
  let mut msg = MSG::default();
  let result = match hwnd {
    Some(hwnd) => unsafe { GetMessageW(&mut msg, Some(hwnd.to_win32()), min, max) },
    None => unsafe { GetMessageW(&mut msg, None, min, max) },
  };
  // WM_QUIT sends return value of zero, causing BOOL to be false. This is still valid though.
  // Only -1 is actually an error.
  match result.0 {
    0 => GetMessageResult::Quit,
    -1 => GetMessageResult::Error(result.ok().unwrap_err()),
    _ => GetMessageResult::Message(ThreadMessage::from(msg)),
  }
}

fn peek_message(
  hwnd: Option<WindowHandle>,
  filter: &Option<RangeInclusive<u32>>,
  flags: PeekMessageFlags,
) -> PeekMessageResult {
  let (min, max) = match filter {
    Some(filter) => (*filter.start(), *filter.end()),
    None => (0, 0),
  };
  let mut msg = MSG::default();
  let result = match hwnd {
    Some(hwnd) => unsafe { PeekMessageW(&mut msg, Some(hwnd.to_win32()), min, max, flags.into()) },
    None => unsafe { PeekMessageW(&mut msg, None, min, max, flags.into()) },
  };
  // If a message is available, the return value is nonzero.
  // If no messages are available, the return value is zero.
  match (result.as_bool(), msg.message) {
    (true, WindowsAndMessaging::WM_QUIT) => PeekMessageResult::Quit,
    (true, _) => PeekMessageResult::Message(ThreadMessage::from(msg)),
    (false, _) => PeekMessageResult::None,
  }
}
