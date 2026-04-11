/*
  Perhaps I should replace this with a more standard enum such as that of winit?
*/

use {
  crate::{
    input::mouse::MouseButton,
    types::Focus,
  },
  dpi::{
    PhysicalPosition,
    PhysicalSize,
    Position,
  },
  keyboard_types::{
    Code,
    Key,
    KeyState,
    Location,
    Modifiers,
  },
  strum::Display,
};

#[derive(Debug, Display, PartialEq, Clone)]
pub enum Event {
  /// Artificial window messages sent by the window loop.
  /// Sent when the message pump is polled, but there are no messages.
  None,
  // /// Sent when the message pump is exiting.
  // LoopExiting,
  /// Messages sent by devices registered for raw input.
  RawInput(RawInputMessage),
  Window(WindowEvent),
}

#[derive(Debug, Display, PartialEq, Clone)]
pub enum WindowEvent {
  // /// Message sent when the window is created.
  // Created,
  // /// Message sent when the window is destroyed.
  // Destroyed,
  /// Message sent when the window X button is pressed.
  CloseRequest,
  /// Message sent when Windows requests the window be repainted.
  Draw,
  /// Message sent when a key is pressed, held, or released.
  Keyboard {
    state: KeyState,
    // Logical key value.
    key: Key,
    // Physical key position. (Use this for games)
    code: Code,
    // Location for keys with multiple instances on common keyboards.
    location: Location,
    // Flags for pressed modifier keys.
    modifiers: Modifiers,
    // True if the key is currently auto-repeated.
    repeat: bool,
    // // Events with this flag should be ignored in a text editor.
    // // and instead [composition events](CompositionEvent) should be used.
    // is_composing: bool,
  },
  ModifiersChanged {
    shift: KeyState,
    ctrl: KeyState,
    alt: KeyState,
    win: KeyState,
  },
  /// Message sent when a mouse button is pressed or released.
  MouseButton {
    button: MouseButton,
    state: KeyState,
    position: Position,
    is_double_click: bool,
  },
  /// Message sent when the scroll wheel is actuated.
  MouseWheel { delta_x: f32, delta_y: f32 },
  /// Message sent when the cursor is moved within the window bounds. Don't
  /// use this for mouse input in cases such as first-person cameras as it is
  /// locked to the bounds of the window.
  CursorMove {
    position: PhysicalPosition<i32>,
    kind: CursorMoveKind,
  },
  /// Message sent when the window is resized. Sent after [`BoundsChanged`]
  Resized(PhysicalSize<u32>),
  /// Message sent when the window is moved. Sent after [`BoundsChanged`]
  Moved(PhysicalPosition<i32>),
  /// Message sent first when the window is moved or resized.
  BoundsChanged {
    outer_position: PhysicalPosition<i32>,
    outer_size: PhysicalSize<u32>,
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

#[derive(Debug, Display, PartialEq, Clone)]
pub enum RawInputMessage {
  /// Raw keyboard input
  Keyboard { physical_key: Code, state: KeyState },
  /// Raw mouse button input
  MouseButton { button: MouseButton, state: KeyState },
  /// Raw mouse motion. Use this for mouse input in cases such as first-person
  /// cameras.
  MouseMove { delta_x: f32, delta_y: f32 },
}

/*
  Adapted from `winit` according to Apache-2.0 license. (https://github.com/rust-windowing/winit/blob/master/src/platform_impl/windows/event_loop.rs#L2568)
  Adapted for Windows crate.
*/
#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CursorMoveKind {
  /// Cursor entered to the window.
  Entered,
  /// Cursor left the window client area.
  Left,
  /// Cursor is inside the window or `GetClientRect` failed.
  Inside,
}
