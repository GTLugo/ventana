use std::collections::HashMap;

use window_input::keyboard::KeyCode;

use self::{
  mouse::MouseButton,
  state::{ButtonState, KeyState},
};

pub mod mouse;
pub mod state;

#[derive(Debug, Default)]
pub struct Input {
  pub mouse_buttons: HashMap<MouseButton, ButtonState>,
  pub keys: HashMap<KeyCode, KeyState>,
  pub shift_key: ButtonState,
  pub ctrl_key: ButtonState,
  pub alt_key: ButtonState,
  pub super_key: ButtonState,
}
