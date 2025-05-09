use std::sync::{Arc, Mutex};

use ventana_hal::{
  context::Context,
  input::{
    mouse::MouseButton,
    state::{ButtonState, KeyState},
  },
  keyboard::KeyCode,
  message::Message,
  position::Position,
  settings::WindowSettings,
  size::Size,
  window::Window as HalWindow,
};

use crate::{
  Error,
  class::WindowClass,
  descriptor::WindowDescriptor,
  handle::window::WindowId,
  message::pump::{MessagePump, PollingMode},
  procedure::{Response, WindowProcedure},
};

pub struct Window {
  hwnd: WindowId,
  settings: Arc<Mutex<WindowSettings>>,
}

impl Window {
  pub fn new(settings: WindowSettings) -> Result<Self, Error> {
    let class = WindowClass::default();
    let hwnd = class.spawn(
      WindowDescriptor::default()
        .with_title(settings.title.clone())
        .with_position(settings.position)
        .with_size(settings.size),
      Internal,
    )?;

    MessagePump::default().with_mode(PollingMode::Poll).run();

    Ok(Self {
      hwnd,
      settings: Arc::new(Mutex::new(settings)),
    })
  }
}

impl HalWindow for Window {
  fn next(&self, context: &Context) -> Option<Message> {
    None
  }

  fn title(&self, context: &Context) -> String {
    self.settings.lock().unwrap().title.clone()
  }

  fn size(&self, context: &Context) -> Size {
    self.settings.lock().unwrap().size
  }

  fn position(&self, context: &Context) -> Position {
    self.settings.lock().unwrap().position
  }

  fn key(&self, context: &Context, keycode: KeyCode) -> KeyState {
    todo!()
  }

  fn mouse(&self, context: &Context, button: MouseButton) -> ButtonState {
    todo!()
  }

  fn shift_key(&self, context: &Context) -> ButtonState {
    todo!()
  }

  fn ctrl_key(&self, context: &Context) -> ButtonState {
    todo!()
  }

  fn alt_key(&self, context: &Context) -> ButtonState {
    todo!()
  }

  fn super_key(&self, context: &Context) -> ButtonState {
    todo!()
  }
}

struct Internal;

impl WindowProcedure for Internal {
  fn on_message(&mut self, mut window: WindowId, message: &crate::message::Message) -> Option<Response> {
    println!("{window:?} | {message:?}");

    // window.destroy(); // This will cause a crash

    None
  }
}
