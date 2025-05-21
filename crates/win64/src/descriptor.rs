use dpi::{Position, Size};

use crate::{
  flag::{ExtendedWindowStyle, WindowStyle},
  types::{Win32Position, Win32Size},
};

#[derive(Debug, Clone, PartialEq)]
pub struct WindowDescriptor {
  pub title: String,
  pub position: Win32Position,
  pub size: Win32Size,
  pub style: WindowStyle,
  pub ext_style: ExtendedWindowStyle,
}

impl Default for WindowDescriptor {
  fn default() -> Self {
    Self {
      title: "Window".to_owned(),
      position: Default::default(),
      size: Default::default(),
      style: WindowStyle::OverlappedWindow | WindowStyle::Visible,
      ext_style: ExtendedWindowStyle::empty(),
    }
  }
}

impl WindowDescriptor {
  pub fn with_title(&mut self, title: impl Into<String>) -> &mut Self {
    self.title = title.into();
    self
  }

  pub fn with_position(&mut self, position: Option<impl Into<Position>>) -> &mut Self {
    self.position = match position {
      Some(pos) => Win32Position::Position(pos.into()),
      None => Win32Position::Auto,
    };
    self
  }

  pub fn with_size(&mut self, size: impl Into<Size>) -> &mut Self {
    self.size = Win32Size::Size(size.into());
    self
  }

  pub fn with_style(&mut self, style: WindowStyle) -> &mut Self {
    self.style = style;
    self
  }

  pub fn with_ext_style(&mut self, ext_style: ExtendedWindowStyle) -> &mut Self {
    self.ext_style = ext_style;
    self
  }
}

// #[cfg(all(feature = "rwh_06", not(feature = "rwh_05")))]
// pub mod rwh_06 {
//   use rwh_06::{
//     DisplayHandle, HandleError, HasDisplayHandle, HasWindowHandle, RawDisplayHandle,
//     RawWindowHandle, Win32WindowHandle, WindowHandle, WindowsDisplayHandle,
//   };

//   use crate::handle::Instance;

//   use super::Window;

//   impl Window {
//     pub fn raw_window_handle(&self) -> RawWindowHandle {
//       let mut handle = Win32WindowHandle::new(
//         std::num::NonZeroIsize::new(self.0.as_ptr() as _)
//           .expect("window handle should not be zero"),
//       );
//       let hinstance =
//         std::num::NonZeroIsize::new(unsafe { Instance::get_exe().as_ptr() } as _)
//           .expect("instance handle should not be zero");
//       handle.hinstance = Some(hinstance);
//       RawWindowHandle::from(handle)
//     }

//     pub fn raw_display_handle(&self) -> RawDisplayHandle {
//       let handle = WindowsDisplayHandle::new();
//       RawDisplayHandle::from(handle)
//     }
//   }

//   impl HasWindowHandle for Window {
//     fn window_handle(&self) -> Result<WindowHandle<'_>, HandleError> {
//       Ok(unsafe { WindowHandle::borrow_raw(self.raw_window_handle()) })
//     }
//   }

//   impl HasDisplayHandle for Window {
//     fn display_handle(&self) -> Result<DisplayHandle<'_>, HandleError> {
//       Ok(unsafe { DisplayHandle::borrow_raw(self.raw_display_handle()) })
//     }
//   }
// }

// #[cfg(all(feature = "rwh_05", not(feature = "rwh_06")))]
// pub mod rwh_05 {
//   use rwh_05::{
//     HasRawDisplayHandle, HasRawWindowHandle, RawDisplayHandle, RawWindowHandle,
//     Win32WindowHandle, WindowsDisplayHandle,
//   };

//   use crate::handle::Instance;

//   use super::Window;

//   unsafe impl HasRawWindowHandle for Window {
//     fn raw_window_handle(&self) -> RawWindowHandle {
//       let mut handle = Win32WindowHandle::empty();
//       handle.hwnd = self.0.as_ptr();
//       handle.hinstance = unsafe { Instance::get_exe().as_ptr() };
//       RawWindowHandle::Win32(handle)
//     }
//   }

//   unsafe impl HasRawDisplayHandle for Window {
//     fn raw_display_handle(&self) -> RawDisplayHandle {
//       RawDisplayHandle::Windows(WindowsDisplayHandle::empty())
//     }
//   }
// }
