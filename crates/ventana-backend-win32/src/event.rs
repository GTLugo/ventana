use {
  ventana_hal::{
    event::WindowEvent,
    keyboard::{
      Key,
      NamedKey,
    },
  },
  win64::user::Message,
};

pub fn map_native_event(native: &Message) -> Option<WindowEvent> {
  Some(match native {
    // Message::Create(_) => WindowEvent::Created,
    // Message::Destroy => WindowEvent::Destroyed,
    Message::Close => WindowEvent::CloseRequest,
    Message::Paint => WindowEvent::Draw,
    Message::KeyDown(message) => {
      let key_event = message.event();
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
    },
    Message::Size(message) => WindowEvent::Resized(message.physical_size()),
    Message::Move(message) => WindowEvent::Moved(message.physical_position()),
    // ...todo
    _ => {
      // log::debug!("{message:?}");
      return None;
    },
  })
}
