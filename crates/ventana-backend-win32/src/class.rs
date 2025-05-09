use cursor_icon::CursorIcon;
use windows::{
  core::{HSTRING, PCWSTR}, Win32::UI::WindowsAndMessaging::{
    CreateWindowExW, GetClassInfoExW, LoadCursorW, RegisterClassExW, UnregisterClassW, WNDCLASSEXW,
  }
};

use crate::{
  descriptor::WindowDescriptor,
  flag::WindowClassStyle,
  handle::{Win32Type, instance::InstanceId, window::WindowId},
  procedure::{self, CreateInfo, WindowProcedure},
  types::ToHCURSOR,
};

const TEMP_SCALE_FACTOR: f64 = 1.0;

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
    let result = unsafe { GetClassInfoExW(Some(instance.to_win32()), &hstring, &mut class) };
    result.map(|_| Self {
      instance: *instance,
      name,
    })
  }

  pub fn unregister(self) -> Result<(), windows::core::Error> {
    let hstring = HSTRING::from(self.name);
    unsafe { UnregisterClassW(PCWSTR(hstring.as_ptr()), Some(self.instance.to_win32())) }?;

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
    let position = desc.position.clone().unwrap_or_default().as_logical(TEMP_SCALE_FACTOR);
    let size = desc.size.clone().unwrap_or_default().as_logical(TEMP_SCALE_FACTOR);
    let instance = self.instance.to_win32();
    let class_name = HSTRING::from(self.name());

    let create_info = Box::into_raw(Box::new(CreateInfo::new(window_state)));

    unsafe {
      CreateWindowExW(
        desc.ext_style.into(),
        &class_name,
        &title,
        desc.style.into(),
        position.x as i32,
        position.y as i32,
        size.width as i32,
        size.height as i32,
        None,
        None,
        Some(instance),
        Some(create_info.cast()),
      )
    }
    .map(Into::into)
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
