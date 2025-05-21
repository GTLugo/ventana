use cursor_icon::CursorIcon;
use dpi::{Pixel, Position, Size};
use windows::Win32::UI::WindowsAndMessaging::{self, CW_USEDEFAULT, HCURSOR};

const INIT_SCALE_FACTOR: f64 = 1.0;

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Win32Position {
  #[default]
  Auto,
  Position(Position),
}

impl Win32Position {
  pub fn x(&self) -> i32 {
    match self {
      Win32Position::Auto => CW_USEDEFAULT,
      Win32Position::Position(Position::Logical(pos)) => pos.to_physical(INIT_SCALE_FACTOR).x,
      Win32Position::Position(Position::Physical(pos)) => pos.x,
    }
  }

  pub fn y(&self) -> i32 {
    match self {
      Win32Position::Auto => CW_USEDEFAULT,
      Win32Position::Position(Position::Logical(pos)) => pos.to_physical(INIT_SCALE_FACTOR).y,
      Win32Position::Position(Position::Physical(pos)) => pos.y,
    }
  }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub enum Win32Size {
  #[default]
  Auto,
  Size(Size),
}

impl Win32Size {
  pub fn width(&self) -> i32 {
    match self {
      Win32Size::Auto => CW_USEDEFAULT,
      Win32Size::Size(Size::Logical(size)) => size.to_physical(INIT_SCALE_FACTOR).width,
      Win32Size::Size(Size::Physical(size)) => size.width.cast(),
    }
  }

  pub fn height(&self) -> i32 {
    match self {
      Win32Size::Auto => CW_USEDEFAULT,
      Win32Size::Size(Size::Logical(size)) => size.to_physical(INIT_SCALE_FACTOR).height,
      Win32Size::Size(Size::Physical(size)) => size.height.cast(),
    }
  }
}

// pub trait Auto {
//   fn auto() -> Self where Self: Sized;
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Size {
//   pub width: i32,
//   pub height: i32,
// }

// impl Size {
//   pub const AUTO: Self = Self {
//     width: CW_USEDEFAULT,
//     height: CW_USEDEFAULT,
//   };
// }

// impl From<[i32; 2]> for Size {
//   fn from([width, height]: [i32; 2]) -> Self {
//     Self { width, height }
//   }
// }

// impl From<(i32, i32)> for Size {
//   fn from((width, height): (i32, i32)) -> Self {
//     Self { width, height }
//   }
// }

// #[derive(Debug, Clone, PartialEq, Eq, Hash)]
// pub struct Position {
//   pub x: i32,
//   pub y: i32,
// }

// impl Position {
//   pub const AUTO: Self = Self {
//     x: CW_USEDEFAULT,
//     y: CW_USEDEFAULT,
//   };
// }

// impl From<[i32; 2]> for Position {
//   fn from([x, y]: [i32; 2]) -> Self {
//     Self { x, y }
//   }
// }

// impl From<(i32, i32)> for Position {
//   fn from((x, y): (i32, i32)) -> Self {
//     Self { x, y }
//   }
// }

pub(crate) trait ToHCURSOR {
  fn to_cursor(&self) -> HCURSOR;
}

impl ToHCURSOR for CursorIcon {
  fn to_cursor(&self) -> HCURSOR {
    match self {
      CursorIcon::Default => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
      CursorIcon::Text => HCURSOR(WindowsAndMessaging::IDC_IBEAM.as_ptr().cast_mut().cast()),
      CursorIcon::Wait => HCURSOR(WindowsAndMessaging::IDC_WAIT.as_ptr().cast_mut().cast()),
      CursorIcon::Help => HCURSOR(WindowsAndMessaging::IDC_HELP.as_ptr().cast_mut().cast()),
      CursorIcon::Crosshair => HCURSOR(WindowsAndMessaging::IDC_CROSS.as_ptr().cast_mut().cast()),
      CursorIcon::NotAllowed => HCURSOR(WindowsAndMessaging::IDC_NO.as_ptr().cast_mut().cast()),
      CursorIcon::Grabbing => HCURSOR(WindowsAndMessaging::IDC_HAND.as_ptr().cast_mut().cast()),
      CursorIcon::Grab => HCURSOR(WindowsAndMessaging::IDC_HAND.as_ptr().cast_mut().cast()),
      CursorIcon::Move => HCURSOR(WindowsAndMessaging::IDC_SIZEALL.as_ptr().cast_mut().cast()),
      CursorIcon::Progress => HCURSOR(WindowsAndMessaging::IDC_APPSTARTING.as_ptr().cast_mut().cast()),
      CursorIcon::VerticalText => HCURSOR(WindowsAndMessaging::IDC_UPARROW.as_ptr().cast_mut().cast()),
      CursorIcon::Alias => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
      CursorIcon::Copy => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
      CursorIcon::Cell => HCURSOR(WindowsAndMessaging::IDC_IBEAM.as_ptr().cast_mut().cast()),
      CursorIcon::ContextMenu => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
      CursorIcon::NoDrop => HCURSOR(WindowsAndMessaging::IDC_NO.as_ptr().cast_mut().cast()),
      CursorIcon::AllScroll => HCURSOR(WindowsAndMessaging::IDC_SIZEALL.as_ptr().cast_mut().cast()),
      CursorIcon::Pointer => HCURSOR(WindowsAndMessaging::IDC_HAND.as_ptr().cast_mut().cast()),
      CursorIcon::EResize => HCURSOR(WindowsAndMessaging::IDC_SIZEWE.as_ptr().cast_mut().cast()),
      CursorIcon::NResize => HCURSOR(WindowsAndMessaging::IDC_SIZENS.as_ptr().cast_mut().cast()),
      CursorIcon::NeResize => HCURSOR(WindowsAndMessaging::IDC_SIZENESW.as_ptr().cast_mut().cast()),
      CursorIcon::NwResize => HCURSOR(WindowsAndMessaging::IDC_SIZENWSE.as_ptr().cast_mut().cast()),
      CursorIcon::SResize => HCURSOR(WindowsAndMessaging::IDC_SIZENS.as_ptr().cast_mut().cast()),
      CursorIcon::SeResize => HCURSOR(WindowsAndMessaging::IDC_SIZENWSE.as_ptr().cast_mut().cast()),
      CursorIcon::SwResize => HCURSOR(WindowsAndMessaging::IDC_SIZENESW.as_ptr().cast_mut().cast()),
      CursorIcon::WResize => HCURSOR(WindowsAndMessaging::IDC_SIZEWE.as_ptr().cast_mut().cast()),
      CursorIcon::EwResize => HCURSOR(WindowsAndMessaging::IDC_SIZEWE.as_ptr().cast_mut().cast()),
      CursorIcon::NsResize => HCURSOR(WindowsAndMessaging::IDC_SIZENS.as_ptr().cast_mut().cast()),
      CursorIcon::NeswResize => HCURSOR(WindowsAndMessaging::IDC_SIZENESW.as_ptr().cast_mut().cast()),
      CursorIcon::NwseResize => HCURSOR(WindowsAndMessaging::IDC_SIZENWSE.as_ptr().cast_mut().cast()),
      CursorIcon::ColResize => HCURSOR(WindowsAndMessaging::IDC_SIZEWE.as_ptr().cast_mut().cast()),
      CursorIcon::RowResize => HCURSOR(WindowsAndMessaging::IDC_SIZENS.as_ptr().cast_mut().cast()),
      CursorIcon::ZoomIn => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
      CursorIcon::ZoomOut => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
      _ => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
    }
  }
}
