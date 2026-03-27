pub mod window;

use {
  self::window::Win32Window,
  std::sync::Arc,
  ventana_hal::{
    backend::Backend,
    error::RequestError,
    event::WindowEvent,
    keyboard::{
      Code,
      Key,
      NamedKey,
    },
    settings::WindowSettings,
    window::BackendWindow,
  },
  win64::user::Message,
};

pub struct Win32;

#[allow(unused)]
impl Backend for Win32 {
  fn instance() -> Arc<dyn Backend>
  where
    Self: Sized,
  {
    Arc::new(Self)
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Win32Window::new(settings)
  }

  fn key_to_scancode(&self, key: Code) -> Option<u32> {
    todo!()
  }

  fn scancode_to_key(&self, scancode: u32) -> Code {
    todo!()
  }
}

fn message_to_event(message: &Message) -> Option<WindowEvent> {
  Some(match message {
    Message::Create(_) => WindowEvent::Created,
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
    // ...todo
    _ => {
      // log::debug!("{message:?}");
      return None;
    },
  })
}
