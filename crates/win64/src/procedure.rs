use crate::descriptor::WindowDescriptor;
use crate::flag::WindowStyle;
use crate::handle::{Handle, Win32Type};
use crate::{
  get_last_error,
  handle::window::WindowHandle,
  message::{Message, data::MessageData, id::MessageId},
};
use windows::Win32::UI::WindowsAndMessaging::{SW_SHOW, ShowWindow};
use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  UI::WindowsAndMessaging::CREATESTRUCTW,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[repr(transparent)]
pub struct Response(pub isize);

impl From<Response> for LRESULT {
  fn from(value: Response) -> Self {
    Self(value.0)
  }
}

impl From<LRESULT> for Response {
  fn from(value: LRESULT) -> Self {
    Self(value.0)
  }
}

pub trait WindowProcedure {
  #[allow(unused_variables)]
  fn on_message(&mut self, window: WindowHandle, message: &Message) -> Option<Response> {
    None
  }
}

pub(crate) struct CreateInfo {
  pub state: Option<Box<dyn WindowProcedure>>,
  pub desc: WindowDescriptor,
}

impl CreateInfo {
  pub fn new(window_state: impl 'static + WindowProcedure, desc: WindowDescriptor) -> Self {
    Self {
      state: Some(Box::new(window_state)),
      desc,
    }
  }

  pub fn from_message(message: Message) -> Self {
    let create_struct = unsafe { (message.raw().l as *mut CREATESTRUCTW).as_mut() }.unwrap();
    let create_info = unsafe { Box::from_raw(create_struct.lpCreateParams as *mut CreateInfo) };
    *create_info
  }
}

pub(crate) enum WindowState {
  Creating(WindowDescriptor),
  Ready,
  Destroying,
}

pub(crate) struct WindowData {
  pub state: WindowState,
  pub proc: Box<dyn WindowProcedure>,
}

impl WindowData {
  pub fn new(mut create_info: CreateInfo) -> Self {
    Self {
      state: WindowState::Creating(create_info.desc),
      proc: create_info.state.take().unwrap(),
    }
  }

  pub fn is_destroying(&self) -> bool {
    matches!(self.state, WindowState::Destroying)
  }
}

/// # Safety
/// Window procedure is inherently unsafe because Win32
pub unsafe extern "system" fn window_procedure(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  let window: WindowHandle = hwnd.into();
  let data = MessageData {
    w: w_param.0,
    l: l_param.0,
  };
  let message = Message::from_raw(msg.into(), data);
  on_message(window, &message)
}

fn on_message(window: WindowHandle, message: &Message) -> LRESULT {
  if !window.is_valid() {
    return LRESULT(0);
  }

  match (window.data(), message.id()) {
    (None, MessageId::NcCreate) => on_nc_create(window, message),
    (Some(data), MessageId::Create) => on_create(window, message, data),
    (Some(_), MessageId::NcDestroy) => {
      window.quit();
      drop(window.take_data());
      LRESULT(0)
    }
    (
      Some(WindowData {
        state: WindowState::Ready,
        proc,
      }),
      _,
    ) => proc
      .on_message(window, message)
      .unwrap_or_else(|| window.default_procedure(message))
      .into(),
    _ => window.default_procedure(message).into(),
  }
}

fn on_nc_create(window: WindowHandle, message: &Message) -> LRESULT {
  let create_info = CreateInfo::from_message(message.clone());

  window.initialize_data(create_info);

  // if let Some(data) = window.data() {
  //   data.proc.on_message(window, message);
  // }

  window.default_procedure(message).into()
}

fn on_create(window: WindowHandle, message: &Message, data: &mut WindowData) -> LRESULT {
  // let create_info = CreateInfo::from_message(message.clone());
  if let WindowState::Creating(desc) = &data.state {
    if desc.style.contains(WindowStyle::Visible) && !unsafe { ShowWindow(window.to_win32(), SW_SHOW) }.as_bool() {
      if let Some(error) = get_last_error() {
        eprintln!("Error: {error}");
        return LRESULT(-1);
      }
    }
  }
  data.state = WindowState::Ready;
  window.default_procedure(message).into()
}
