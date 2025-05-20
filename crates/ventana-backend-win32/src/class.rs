use cursor_icon::CursorIcon;
use ventana_hal::dpi::{Position, Size};
use windows::{
  core::{HSTRING, PCWSTR}, Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, GetClassInfoExW, LoadCursorW, RegisterClassExW, UnregisterClassW, WNDCLASSEXW,
  }
};
use windows::Win32::UI::WindowsAndMessaging::CW_USEDEFAULT;
use ventana_hal::settings::WindowSettings;
use crate::{descriptor::WindowDescriptor, flag::WindowClassStyle, handle::{Win32Type, instance::InstanceId, window::WindowHandle}, procedure::{self, CreateInfo, WindowProcedure}, types::ToHCURSOR, Error};

const TEMP_SCALE_FACTOR: f64 = 1.0;

#[derive(Debug, Clone, PartialEq)]
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
    let hs = HSTRING::from(name.clone());
    let mut class = WNDCLASSEXW::default();
    let result = unsafe { GetClassInfoExW(Some(instance.to_win32()), &hs, &mut class) };
    result.map(|_| Self {
      instance: *instance,
      name,
    })
  }

  pub fn unregister(self) -> Result<(), windows::core::Error> {
    let hs = HSTRING::from(self.name);
    unsafe { UnregisterClassW(PCWSTR(hs.as_ptr()), Some(self.instance.to_win32())) }?;

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
    settings: WindowSettings,
    desc: &WindowDescriptor,
    window_state: impl 'static + WindowProcedure,
  ) -> Result<WindowHandle, Error> {
    let title = HSTRING::from(desc.title.clone());
    
    let (pos_x, pos_y) = match desc.position { 
      Some(Position::Logical(pos)) => {
        let pos = pos.cast();
        (pos.x, pos.y)
      },
      Some(Position::Physical(pos)) => {
        let pos = pos.to_logical(TEMP_SCALE_FACTOR);
        (pos.x, pos.y)
      },
      None => (CW_USEDEFAULT, CW_USEDEFAULT),
    };
    let size = desc.size.unwrap_or(Size::Logical((0.0, 0.0).into())).to_logical(TEMP_SCALE_FACTOR);
    let instance = self.instance.to_win32();
    let class_name = HSTRING::from(self.name());
    
    let create_info = Box::into_raw(Box::new(CreateInfo::new(window_state, settings)));
    
    match unsafe {
      CreateWindowExW(
        desc.ext_style.into(),
        &class_name,
        &title,
        desc.style.into(),
        pos_x,
        pos_y,
        size.width,
        size.height,
        None,
        None,
        Some(instance),
        Some(create_info.cast()),
      )
    } {
      Ok(hwnd) => {
        Ok(hwnd.into())
      }
      Err(err) => {
        Err(Error::Win32Error(err))
      }
    }
  }
}

impl Default for WindowClass {
  fn default() -> Self {
    Self::new(&WindowClassDescriptor::default())
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
