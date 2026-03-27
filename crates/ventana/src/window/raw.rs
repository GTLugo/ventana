#[cfg(raw_window_handle_v5)]
use rwh_05::{
  HasRawDisplayHandle,
  HasRawWindowHandle,
  RawDisplayHandle,
  RawWindowHandle,
};
#[cfg(raw_window_handle_v6)]
use rwh_06::{
  DisplayHandle,
  HandleError,
  HasDisplayHandle,
  HasWindowHandle,
  WindowHandle,
};

use super::Window;

#[cfg(raw_window_handle_v5)]
unsafe impl HasRawWindowHandle for Window {
  fn raw_window_handle(&self) -> RawWindowHandle {
    self.window.raw_window_handle()
  }
}

#[cfg(raw_window_handle_v5)]
unsafe impl HasRawDisplayHandle for Window {
  fn raw_display_handle(&self) -> RawDisplayHandle {
    self.window.raw_display_handle()
  }
}

#[cfg(raw_window_handle_v6)]
impl HasWindowHandle for Window {
  fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
    Ok(unsafe { WindowHandle::borrow_raw(self.window.raw_window_handle()) })
  }
}

#[cfg(raw_window_handle_v6)]
impl HasDisplayHandle for Window {
  fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
    Ok(unsafe { DisplayHandle::borrow_raw(self.window.raw_display_handle()) })
  }
}
