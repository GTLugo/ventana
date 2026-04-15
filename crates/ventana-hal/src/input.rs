use {
  keyboard_types::{
    Code,
    KeyState,
  },
  pointer_types::{
    ButtonState,
    mouse::MouseButton,
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
