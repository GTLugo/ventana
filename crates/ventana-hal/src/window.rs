use dpi::{Position, Size};
use keyboard_types::{Code, KeyState};

use crate::{
  event::Event,
  input::{
    mouse::MouseButton,
  },
};
use crate::context::Backend;

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

  fn next(&self, context: &dyn Backend) -> Option<Event>;

  fn title(&self, context: &dyn Backend) -> String;

  fn size(&self, context: &dyn Backend) -> Size;

  fn position(&self, context: &dyn Backend) -> Position;

  fn key(&self, context: &dyn Backend, keycode: Code) -> KeyState;

  fn mouse(&self, context: &dyn Backend, button: MouseButton) -> KeyState;

  fn shift_key(&self, context: &dyn Backend) -> KeyState;

  fn ctrl_key(&self, context: &dyn Backend) -> KeyState;

  fn alt_key(&self, context: &dyn Backend) -> KeyState;

  fn super_key(&self, context: &dyn Backend) -> KeyState;
}
