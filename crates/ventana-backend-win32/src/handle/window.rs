use std::ptr::NonNull;

use windows::{
  Win32::{
    Foundation::{HWND, LPARAM, SetLastError, WIN32_ERROR, WPARAM},
    UI::WindowsAndMessaging::{
      DefWindowProcW, DestroyWindow, GetWindowLongPtrW, PostQuitMessage, SetWindowLongPtrW, SetWindowTextW,
    },
  },
  core::{HRESULT, HSTRING},
};

use crate::{
  flag::LongPointerIndex,
  message::Message,
  procedure::{CreateInfo, Response, State, WindowData},
};

use super::{Handle, Win32Type};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(Option<NonNull<usize>>)
where
  Self: Send + Sync;

unsafe impl Send for WindowId {}
unsafe impl Sync for WindowId {}

type Void = core::ffi::c_void;

impl Handle for WindowId {
  fn as_ptr(&self) -> *mut Void {
    self.0.map_or(core::ptr::null_mut(), |ptr| ptr.as_ptr().cast())
  }

  unsafe fn from_ptr(ptr: *mut Void) -> Self {
    let ptr: *mut usize = ptr.cast();
    let hwnd = match ptr.is_null() {
      true => None,
      false => Some(unsafe { NonNull::new_unchecked(ptr) }),
    };
    Self(hwnd)
  }

  fn is_valid(&self) -> bool {
    self.0.is_some()
  }
}

impl Win32Type for WindowId {
  type Type = HWND;

  fn to_win32(&self) -> Self::Type {
    (*self).into()
  }
}

impl WindowId {
  pub fn send_message(&self) {
    // TODO: somehow ensure these are always sent to the correct thread, even when called from a different thread.
    // maybe do it by storing the thread id?
    todo!()
  }

  pub fn default_procedure(&self, message: &Message) -> Response {
    unsafe { DefWindowProcW(self.to_win32(), message.id().to_u32(), WPARAM(message.raw().w), LPARAM(message.raw().l)) }
      .into()
  }

  pub fn destroy(&mut self) {
    if self.is_valid() {
      if let Some(data) = self.data() {
        data.state = State::Destroyed;
      }
      unsafe { DestroyWindow(self.to_win32()) }.unwrap();
    }
    self.0.take();
  }

  pub(crate) fn quit(&self) {
    self.quit_with_code(0)
  }

  pub(crate) fn quit_with_code(&self, exit_code: i32) {
    unsafe { PostQuitMessage(exit_code) };
  }

  pub fn set_window_text(&self, text: impl Into<String>) -> windows::core::Result<()> {
    let text = HSTRING::from(text.into());
    unsafe { SetWindowTextW(self.to_win32(), &text) }
  }

  // TODO: Migrate these functions to use conditionals for diff ptr widths
  pub(crate) fn get_window_long(&self, index: LongPointerIndex) -> isize {
    unsafe { GetWindowLongPtrW(self.to_win32(), index.to_win32()) }
  }

  pub(crate) fn set_window_long(&self, index: LongPointerIndex, value: isize) -> Option<isize> {
    unsafe { SetLastError(WIN32_ERROR::default()) }; // clear error
    let result = unsafe { SetWindowLongPtrW(self.to_win32(), index.to_win32(), value) };
    let error = windows::core::Error::from_win32();
    match result == 0 && error.code() != HRESULT(0) {
      true => {
        eprintln!("Error: {}", windows::core::Error::from_win32());
        None
      }
      false => Some(result),
    }
  }

  pub(crate) fn initialize_data(&self, create_info: CreateInfo) {
    let data = WindowData::new(create_info);
    let data_ptr = Box::into_raw(Box::new(data));
    self.set_window_long(LongPointerIndex::UserData, data_ptr as isize);
  }

  pub(crate) fn data(&self) -> Option<&mut WindowData> {
    unsafe { (self.get_window_long(LongPointerIndex::UserData) as *mut WindowData).as_mut() }
  }

  pub(crate) fn take_data(&self) -> Box<WindowData> {
    unsafe { Box::from_raw(self.get_window_long(LongPointerIndex::UserData) as *mut WindowData) }
  }
}

impl From<WindowId> for HWND {
  fn from(value: WindowId) -> Self {
    Self(value.as_ptr())
  }
}

impl From<HWND> for WindowId {
  fn from(value: HWND) -> Self {
    unsafe { Self::from_ptr(value.0) }
  }
}
