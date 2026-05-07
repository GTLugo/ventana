pub mod key;

use {
  keyboard_types::{
    Code,
    KeyState,
  },
  pointer_types::{
    ButtonState,
    PointerButton,
  },
  std::collections::HashMap,
};

#[derive(Debug)]
pub struct Input {
  pub pointer_buttons: HashMap<PointerButton, ButtonState>,
  pub keys: HashMap<Code, KeyState>,
  pub shift_key: KeyState,
  pub ctrl_key: KeyState,
  pub alt_key: KeyState,
  pub meta_key: KeyState,
}

impl Default for Input {
  fn default() -> Self {
    Self {
      pointer_buttons: Default::default(),
      keys: Default::default(),
      shift_key: KeyState::Up,
      ctrl_key: KeyState::Up,
      alt_key: KeyState::Up,
      meta_key: KeyState::Up,
    }
  }
}

impl Input {
  pub fn update_key(&mut self, code: Code, state: KeyState) {
    match code {
      Code::ShiftLeft | Code::ShiftRight => {
        self.shift_key = state;
      },
      Code::ControlLeft | Code::ControlRight => {
        self.ctrl_key = state;
      },
      Code::AltLeft | Code::AltRight => {
        self.alt_key = state;
      },
      Code::MetaLeft | Code::MetaRight => {
        self.meta_key = state;
      },
      _ => {
        self.keys.insert(code, state);
      },
    }
  }

  pub fn update_pointer(&mut self, button: PointerButton, state: ButtonState) {
    self.pointer_buttons.insert(button, state);
  }

  pub fn key(&self, code: Code) -> KeyState {
    *self.keys.get(&code).unwrap_or(&KeyState::Up)
  }

  pub fn pointer(&self, button: PointerButton) -> ButtonState {
    *self.pointer_buttons.get(&button).unwrap_or(&ButtonState::Up)
  }

  pub fn shift_key(&self) -> KeyState {
    self.shift_key
  }

  pub fn ctrl_key(&self) -> KeyState {
    self.ctrl_key
  }

  pub fn alt_key(&self) -> KeyState {
    self.alt_key
  }

  pub fn meta_key(&self) -> KeyState {
    self.meta_key
  }
}
