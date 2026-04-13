#![cfg(target_os = "windows")]

use {
  ventana_hal::{
    event::WindowEvent,
    keyboard::{
      Key,
      NamedKey,
    },
    mouse::event::MouseEvent,
  },
  win64::user::{
    KeyEvent,
    Message,
  },
};

pub fn map_native_event(native: &Message) -> Option<WindowEvent> {
  Some(match native {
    // Message::Create(_) => WindowEvent::Created,
    // Message::Destroy => WindowEvent::Destroyed,
    Message::Close => WindowEvent::CloseRequest,
    Message::Paint => WindowEvent::Draw,
    Message::KeyDown(message) => key_event_to_window_event(message.event()),
    Message::KeyUp(message) => key_event_to_window_event(message.event()),
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

fn key_event_to_window_event(key_event: KeyEvent) -> WindowEvent {
  use win64::input::keyboard::key::Key as WinKey;
  let key = match key_event.key {
    WinKey::Named(named_key) => Key::Named(named_key),
    WinKey::Character(string) => Key::Character(string),
    WinKey::Unidentified(_) => Key::Named(NamedKey::Unidentified),
    WinKey::Dead(_) => Key::Named(NamedKey::Dead),
  };
  WindowEvent::Keyboard {
    state: key_event.state,
    key,
    code: key_event.code,
    location: key_event.location,
    modifiers: key_event.modifiers,
    repeat: key_event.repeat,
    // is_composing: key_event,
  }
}

fn mouse_event_to_window_event(mouse_event: MouseEvent) -> WindowEvent {
  WindowEvent::MouseButton {
    button: mouse_event.button,
    state: mouse_event.state,
    position: mouse_event.position,
    is_double_click: mouse_event.is_double_click,
  }
}
