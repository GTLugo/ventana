use {
  keyboard_types::{
    Code,
    KeyState,
  },
  mouse_types::{
    button::MouseButton,
    state::ButtonState,
  },
  std::collections::HashMap,
};

#[derive(Debug, Default)]
pub struct Input {
  pub mouse_buttons: HashMap<MouseButton, ButtonState>,
  pub keys: HashMap<Code, KeyState>,
  pub shift_key: KeyState,
  pub ctrl_key: KeyState,
  pub alt_key: KeyState,
  pub super_key: KeyState,
}
