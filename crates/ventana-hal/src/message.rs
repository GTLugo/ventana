use window_input::keyboard::{Key, KeyLocation, PhysicalKey, SmolStr};

/*
  Perhaps I should replace this with a more standard enum such as that of winit?
*/

use crate::{
  input::{
    mouse::MouseButton,
    state::{ButtonState, KeyState, RawKeyState},
  },
  position::PhysicalPosition,
  size::PhysicalSize,
  types::Focus,
};

#[derive(Debug, PartialEq, Clone)]
pub enum Message {
  /// Artificial window messages sent by the window loop.
  Loop(LoopMessage),
  /// Messages sent by devices registered for raw input.
  RawInput(RawInputMessage),
  /// Message sent when window is created.
  Created { hwnd: usize, hinstance: usize },
  /// Message sent when window X button is pressed.
  CloseRequested,
  /// Message sent when Windows requests the window be repainted.
  Paint,
  /// Message sent when a key is pressed, held, or released.
  Key {
    physical_key: PhysicalKey,
    logical_key: Key,
    text: Option<SmolStr>,
    state: KeyState,
    location: KeyLocation,
  },
  /// Message sent when a text character is typed containing that character.
  Text(String),
  ModifiersChanged {
    shift: ButtonState,
    ctrl: ButtonState,
    alt: ButtonState,
    win: ButtonState,
  },
  /// Message sent when a mouse button is pressed or released.
  MouseButton {
    button: MouseButton,
    state: ButtonState,
    position: PhysicalPosition,
    is_double_click: bool,
  },
  /// Message sent when the scroll wheel is actuated.
  MouseWheel { delta_x: f32, delta_y: f32 },
  /// Message sent when the cursor is moved within the window bounds. Don't
  /// use this for mouse input in cases such as first-person cameras as it is
  /// locked to the bounds of the window.
  CursorMove {
    position: PhysicalPosition,
    kind: CursorMoveKind,
  },
  /// Message sent when the window is resized. Sent after [`BoundsChanged`]
  Resized(PhysicalSize),
  /// Message sent when the window is moved. Sent after [`BoundsChanged`]
  Moved(PhysicalPosition),
  /// Message sent first when the window is moved or resized.
  BoundsChanged {
    outer_position: PhysicalPosition,
    outer_size: PhysicalSize,
  },
  /// Message sent by Windows when certain actions are taken. WIP
  Command,
  /// Message sent by Windows when certain actions are taken. WIP
  SystemCommand,
  /// Message sent when the window gains or loses focus.
  Focus(Focus),
  /// Message sent when the scale factor of the window has changed.
  ScaleFactorChanged(f64),
}

pub struct KeyMessage {

}

/// Artificial window messages sent by the window loop.
#[derive(Debug, PartialEq, Clone)]
pub enum LoopMessage {
  /// Sent when the window receives a command request.
  Command, /*(Command)*/
  /// Sent when the message pump is polled, but there are no messages.
  Empty,
  /// Sent when the message pump is exiting.
  Exit,
}

#[derive(Debug, PartialEq, Clone)]
pub enum RawInputMessage {
  /// Raw keyboard input
  Keyboard {
    physical_key: PhysicalKey,
    state: RawKeyState,
  },
  /// Raw mouse button input
  MouseButton { button: MouseButton, state: ButtonState },
  /// Raw mouse motion. Use this for mouse input in cases such as first-person
  /// cameras.
  MouseMove { delta_x: f32, delta_y: f32 },
}

/*
  Adapted from `winit` according to Apache-2.0 license. (https://github.com/rust-windowing/winit/blob/master/src/platform_impl/windows/event_loop.rs#L2568)
  Adapted for windows crate.
*/
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CursorMoveKind {
  /// Cursor entered to the window.
  Entered,
  /// Cursor left the window client area.
  Left,
  /// Cursor is inside the window or `GetClientRect` failed.
  Inside,
}
