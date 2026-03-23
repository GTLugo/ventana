use {
  crate::{
    event::Event,
    input::mouse::MouseButton,
  },
  dpi::{
    Position,
    Size,
  },
  keyboard_types::{
    Code,
    KeyState,
  },
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(usize);

impl WindowId {
  pub const fn to_raw(self) -> usize {
    self.0
  }

  pub const fn from_raw(raw: usize) -> Self {
    Self(raw)
  }
}

// Maybe split this into things like "BasicWindow" "ResizableWindow" "MoveableWindow" etc
pub trait BackendWindow: Send + Sync {
  fn id(&self) -> WindowId;

  fn next(&self) -> Option<Event>;

  fn title(&self) -> String;

  fn size(&self) -> Size;

  fn position(&self) -> Position;

  fn key(&self, keycode: Code) -> KeyState;

  fn mouse(&self, button: MouseButton) -> KeyState;

  fn shift_key(&self) -> KeyState;

  fn ctrl_key(&self) -> KeyState;

  fn alt_key(&self) -> KeyState;

  fn super_key(&self) -> KeyState;
}
