use cursor_icon::CursorIcon;
use windows::{
  core::{HSTRING, PCWSTR},
  Win32::UI::WindowsAndMessaging::{
    self, CreateWindowExW, GetClassInfoExW, LoadCursorW, RegisterClassExW, UnregisterClassW, CW_USEDEFAULT, HCURSOR,
    WNDCLASSEXW,
  },
};

use crate::{
  flag::WindowClassStyle,
  handle::{instance::InstanceId, window::WindowId, Win32Type},
  prelude::{WindowDescriptor, WindowProcedure},
  procedure::{self, CreateInfo},
};

pub struct WindowClass {
  instance: InstanceId,
  name: String,
}

impl WindowClass {
  pub fn new(desc: &WindowClassDescriptor) -> Self {
    let name = HSTRING::from(desc.name.clone());
    let wc = WNDCLASSEXW {
      cbSize: core::mem::size_of::<WNDCLASSEXW>() as _,
      hInstance: desc.instance.to_win32(),
      lpszClassName: PCWSTR(name.as_ptr()),
      lpfnWndProc: Some(procedure::window_procedure),
      style: desc.style.into(),
      hCursor: unsafe { LoadCursorW(None, PCWSTR(desc.cursor.to_cursor().0.cast())).unwrap() },
      ..Default::default()
    };

    unsafe { RegisterClassExW(&wc) };

    Self {
      instance: desc.instance,
      name: desc.name.clone(),
    }
  }

  pub fn get(instance: &InstanceId, name: String) -> Result<Self, windows::core::Error> {
    let hstring = HSTRING::from(name.clone());
    let mut class = WNDCLASSEXW::default();
    let result = unsafe { GetClassInfoExW(instance.to_win32(), &hstring, &mut class) };
    result.map(|_| Self {
      instance: *instance,
      name,
    })
  }

  pub fn unregister(self) -> Result<(), windows::core::Error> {
    let hstring = HSTRING::from(self.name);
    unsafe { UnregisterClassW(PCWSTR(hstring.as_ptr()), self.instance.to_win32()) }?;

    Ok(())
  }

  pub fn name(&self) -> &String {
    &self.name
  }

  pub fn instance(&self) -> &InstanceId {
    &self.instance
  }

  pub fn spawn(
    &self,
    desc: &WindowDescriptor,
    window_state: impl 'static + WindowProcedure,
  ) -> Result<WindowId, windows::core::Error> {
    let title = HSTRING::from(desc.title.clone());
    let mut create_info = CreateInfo {
      state: Some(Box::new(window_state)),
    };
    let position = desc.position.clone().unwrap_or(Position::AUTO);
    let size = desc.size.clone().unwrap_or(Size::AUTO);
    let instance = self.instance.to_win32();
    let class_name = HSTRING::from(self.name());

    unsafe {
      CreateWindowExW(
        desc.ext_style.into(),
        &class_name,
        &title,
        desc.style.into(),
        position.x,
        position.y,
        size.width,
        size.height,
        None,
        None,
        instance,
        Some(std::ptr::addr_of_mut!(create_info).cast()),
      )
    }
    .map(Into::into)
  }
}

pub struct WindowClassDescriptor {
  pub instance: InstanceId,
  pub name: String,
  pub style: WindowClassStyle,
  pub cursor: CursorIcon,
}

impl Default for WindowClassDescriptor {
  fn default() -> Self {
    Self {
      instance: Default::default(),
      name: "Window Class".to_owned(),
      style: WindowClassStyle::HorizontalRedraw | WindowClassStyle::VerticalRedraw,
      cursor: Default::default(),
    }
  }
}

impl WindowClassDescriptor {
  pub fn with_instance(&mut self, instance: InstanceId) -> &mut Self {
    self.instance = instance;
    self
  }

  pub fn with_name(&mut self, name: impl Into<String>) -> &mut Self {
    self.name = name.into();
    self
  }

  pub fn with_style(&mut self, style: WindowClassStyle) -> &mut Self {
    self.style = style;
    self
  }

  pub fn with_cursor(&mut self, cursor: CursorIcon) -> &mut Self {
    self.cursor = cursor;
    self
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Size {
  pub width: i32,
  pub height: i32,
}

impl Size {
  pub const AUTO: Self = Self {
    width: CW_USEDEFAULT,
    height: CW_USEDEFAULT,
  };
}

impl From<[i32; 2]> for Size {
  fn from([width, height]: [i32; 2]) -> Self {
    Self { width, height }
  }
}

impl From<(i32, i32)> for Size {
  fn from((width, height): (i32, i32)) -> Self {
    Self { width, height }
  }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
  pub x: i32,
  pub y: i32,
}

impl Position {
  pub const AUTO: Self = Self {
    x: CW_USEDEFAULT,
    y: CW_USEDEFAULT,
  };
}

impl From<[i32; 2]> for Position {
  fn from([x, y]: [i32; 2]) -> Self {
    Self { x, y }
  }
}

impl From<(i32, i32)> for Position {
  fn from((x, y): (i32, i32)) -> Self {
    Self { x, y }
  }
}

trait ToHCURSOR {
  fn to_cursor(&self) -> HCURSOR;
}

impl ToHCURSOR for CursorIcon {
  fn to_cursor(&self) -> HCURSOR {
    match self {
      CursorIcon::Default => HCURSOR(WindowsAndMessaging::IDC_ARROW.as_ptr().cast_mut().cast()),
      CursorIcon::ContextMenu => todo!(),
      CursorIcon::Help => todo!(),
      CursorIcon::Pointer => todo!(),
      CursorIcon::Progress => todo!(),
      CursorIcon::Wait => todo!(),
      CursorIcon::Cell => todo!(),
      CursorIcon::Crosshair => todo!(),
      CursorIcon::Text => todo!(),
      CursorIcon::VerticalText => todo!(),
      CursorIcon::Alias => todo!(),
      CursorIcon::Copy => todo!(),
      CursorIcon::Move => todo!(),
      CursorIcon::NoDrop => todo!(),
      CursorIcon::NotAllowed => todo!(),
      CursorIcon::Grab => todo!(),
      CursorIcon::Grabbing => todo!(),
      CursorIcon::EResize => todo!(),
      CursorIcon::NResize => todo!(),
      CursorIcon::NeResize => todo!(),
      CursorIcon::NwResize => todo!(),
      CursorIcon::SResize => todo!(),
      CursorIcon::SeResize => todo!(),
      CursorIcon::SwResize => todo!(),
      CursorIcon::WResize => todo!(),
      CursorIcon::EwResize => todo!(),
      CursorIcon::NsResize => todo!(),
      CursorIcon::NeswResize => todo!(),
      CursorIcon::NwseResize => todo!(),
      CursorIcon::ColResize => todo!(),
      CursorIcon::RowResize => todo!(),
      CursorIcon::AllScroll => todo!(),
      CursorIcon::ZoomIn => todo!(),
      CursorIcon::ZoomOut => todo!(),
      _ => todo!(),
    }
  }
}
