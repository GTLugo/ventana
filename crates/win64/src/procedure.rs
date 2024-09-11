use windows::Win32::{
  Foundation::{HWND, LPARAM, LRESULT, WPARAM},
  UI::WindowsAndMessaging::{self, DefWindowProcW, GetWindowLongPtrW, SetWindowLongPtrW, CREATESTRUCTW},
};

use crate::{handle::window::WindowId, message::Message, ProcedureResult};

pub trait WindowProcedure {
  fn on_message(&mut self, window: WindowId, message: Message) -> ProcedureResult {
    window.default_procedure(message)
  }
}

pub(crate) struct CreateInfo {
  pub state: Option<Box<dyn WindowProcedure>>,
}

struct WindowState {
  state: Box<dyn WindowProcedure>,
}

/// # Safety
/// Window procedure is inherently unsafe because Win32
pub unsafe extern "system" fn window_procedure(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  let state_ptr = unsafe { GetWindowLongPtrW(hwnd, WindowsAndMessaging::GWLP_USERDATA) };
  let state = unsafe { (state_ptr as *mut WindowState).as_mut() };
  on_message(hwnd, msg, w_param, l_param, state)
}

fn on_nccreate(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM) -> LRESULT {
  let create_struct = unsafe { (l_param.0 as *mut CREATESTRUCTW).as_mut().unwrap() };
  let create_info = unsafe { (create_struct.lpCreateParams as *mut CreateInfo).as_mut().unwrap() };

  let state = WindowState {
    state: create_info.state.take().unwrap(),
  };
  let state_ptr = Box::into_raw(Box::new(state));

  unsafe { SetWindowLongPtrW(hwnd, WindowsAndMessaging::GWLP_USERDATA, state_ptr as isize) };
  let state = unsafe { state_ptr.as_mut() };

  state
    .unwrap()
    .state
    .on_message(hwnd.into(), Message::new(msg, w_param, l_param));

  unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) }
}

fn on_message(hwnd: HWND, msg: u32, w_param: WPARAM, l_param: LPARAM, state: Option<&mut WindowState>) -> LRESULT {
  match (state, msg) {
    (None, WindowsAndMessaging::WM_NCCREATE) => on_nccreate(hwnd, msg, w_param, l_param),
    (Some(state), WindowsAndMessaging::WM_NCDESTROY) => {
      let state_ptr = unsafe { GetWindowLongPtrW(hwnd, WindowsAndMessaging::GWLP_USERDATA) };
      let _state = unsafe { Box::from_raw(state_ptr as *mut WindowState) }; // keep it alive until the end of the block
      if unsafe { SetWindowLongPtrW(hwnd, WindowsAndMessaging::GWLP_USERDATA, 0) } == 0 {
        eprintln!("Error: {}", windows::core::Error::from_win32());
      }
      state
        .state
        .on_message(hwnd.into(), Message::new(msg, w_param, l_param))
        .into()
    }
    (Some(state), _) => {
      // ...
      state
        .state
        .on_message(hwnd.into(), Message::new(msg, w_param, l_param))
        .into()
    }
    _ => unsafe { DefWindowProcW(hwnd, msg, w_param, l_param) },
  }
}
