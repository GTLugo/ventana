use window_input::keyboard::KeyCode;

use crate::{
  context::Context, input::{
    mouse::MouseButton,
    state::{ButtonState, KeyState},
  }, message::Message, position::Position, size::Size
};

// Maybe split this into things like "BasicWindow" "ResizableWindow" "MoveableWindow" etc
pub trait Window: Send + Sync {
  // fn id(&self) -> WindowId;

  fn next(&self, context: &Context) -> Option<Message>;

  fn title(&self, context: &Context) -> String;

  fn size(&self, context: &Context) -> Size;

  fn position(&self, context: &Context) -> Position;

  fn key(&self, context: &Context, keycode: KeyCode) -> KeyState;

  fn mouse(&self, context: &Context, button: MouseButton) -> ButtonState;

  fn shift_key(&self, context: &Context) -> ButtonState;

  fn ctrl_key(&self, context: &Context) -> ButtonState;

  fn alt_key(&self, context: &Context) -> ButtonState;

  fn super_key(&self, context: &Context) -> ButtonState;
}
