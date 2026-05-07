#![cfg(windows)]

use {
  ventana_hal::{
    event::{
      KeyEvent,
      WindowEvent,
    },
    input::key::{
      LogicalKey,
      NativeKey,
      PhysicalKey,
    },
    pointer::mouse::MouseEvent,
  },
  win64::user::{
    KeyEvent as WinKeyEvent,
    Message,
  },
};

pub fn map_native_event(native: &Message) -> Option<WindowEvent> {
  Some(match native {
    // Message::Create(_) => WindowEvent::Created,
    // Message::Destroy => WindowEvent::Destroyed,
    Message::Close => WindowEvent::CloseRequest,
    Message::Paint => WindowEvent::Draw,
    Message::KeyDown(message) => WindowEvent::Keyboard(convert_key_event(message.event())),
    Message::KeyUp(message) => WindowEvent::Keyboard(convert_key_event(message.event())),
    Message::LButtonDown(message) => mouse_event_to_window_event(message.event()),
    Message::LButtonUp(message) => mouse_event_to_window_event(message.event()),
    Message::LButtonDblClk(message) => mouse_event_to_window_event(message.event()),
    Message::RButtonDown(message) => mouse_event_to_window_event(message.event()),
    Message::RButtonUp(message) => mouse_event_to_window_event(message.event()),
    Message::RButtonDblClk(message) => mouse_event_to_window_event(message.event()),
    Message::MButtonDown(message) => mouse_event_to_window_event(message.event()),
    Message::MButtonUp(message) => mouse_event_to_window_event(message.event()),
    Message::MButtonDblClk(message) => mouse_event_to_window_event(message.event()),
    Message::XButtonDown(message) => mouse_event_to_window_event(message.event()),
    Message::XButtonUp(message) => mouse_event_to_window_event(message.event()),
    Message::XButtonDblClk(message) => mouse_event_to_window_event(message.event()),
    Message::Size(message) => WindowEvent::Resized(message.physical_size()),
    Message::Move(message) => WindowEvent::Moved(message.physical_position()),
    // ...todo
    _ => {
      // log::debug!("{message:?}");
      return None;
    },
  })
}

fn convert_key_event(key_event: WinKeyEvent) -> KeyEvent {
  use win64::input::keyboard::key::Key as WinKey;
  let key = match key_event.key {
    WinKey::Named(named_key) => LogicalKey::Named(named_key),
    WinKey::Character(string) => LogicalKey::Character(string.into()),
    WinKey::Unidentified(vkey) => LogicalKey::Unidentified(NativeKey::Key(vkey as u32)),
    WinKey::Dead(dead) => LogicalKey::Dead(dead),
  };
  KeyEvent {
    state: key_event.state,
    key,
    code: PhysicalKey::Code(key_event.code),
    location: key_event.location,
    modifiers: key_event.modifiers,
    repeat: key_event.repeat,
    // is_composing: key_event,
  }
}

fn mouse_event_to_window_event(mouse_event: MouseEvent) -> WindowEvent {
  WindowEvent::Pointer(mouse_event.into())
}
