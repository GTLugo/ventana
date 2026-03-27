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

impl std::fmt::Display for WindowId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

// Maybe split this into things like "BasicWindow" "ResizableWindow" "MoveableWindow" etc
pub trait BackendWindow: Send + Sync {
  fn id(&self) -> WindowId;

  fn next_event(&self) -> Option<Event>;

  fn iter<'w>(&'w self) -> Box<dyn BackendEventIterator<'w> + 'w>;

  fn close(&self);

  fn is_closing(&self) -> bool;

  fn title(&self) -> String;

  fn inner_size(&self) -> Size;

  fn outer_size(&self) -> Size;

  fn inner_position(&self) -> Position;

  fn outer_position(&self) -> Position;

  fn key(&self, keycode: Code) -> KeyState;

  fn mouse(&self, button: MouseButton) -> KeyState;

  fn shift_key(&self) -> KeyState;

  fn ctrl_key(&self) -> KeyState;

  fn alt_key(&self) -> KeyState;

  fn super_key(&self) -> KeyState;
}

pub trait BackendEventIterator<'window>: Send + Sync {
  fn next(&mut self) -> Option<Event>;
}
