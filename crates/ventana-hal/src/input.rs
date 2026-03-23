pub mod mouse;

use {
  self::mouse::MouseButton,
  keyboard_types::{
    Code,
    KeyState,
  },
  std::collections::HashMap,
};

#[derive(Debug, Default)]
pub struct Input {
  pub mouse_buttons: HashMap<MouseButton, KeyState>,
  pub keys: HashMap<Code, KeyState>,
  pub shift_key: KeyState,
  pub ctrl_key: KeyState,
  pub alt_key: KeyState,
  pub super_key: KeyState,
}
