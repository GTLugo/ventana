use std::sync::{Arc, Mutex};
use ventana_hal::{
  dpi::{Position, Size},
  event::Event,
  input::mouse::MouseButton,
  keyboard::{Code, KeyState},
  settings::WindowSettings,
  window::{BackendWindow, WindowId},
};
use ventana_hal::context::Backend;
use crate::{
  Error,
  class::WindowClass,
  descriptor::WindowDescriptor,
  handle::{Handle, window::WindowHandle},
  message::pump::{MessagePump, PollingMode},
  procedure::{Response, WindowProcedure},
};

pub struct Window {
  hwnd: WindowHandle,
  settings: Arc<Mutex<WindowSettings>>,
}

impl Window {
  pub fn new(settings: WindowSettings) -> Result<Self, Error> {
    let class = WindowClass::default();
    let hwnd = class.spawn(
      settings.clone(),
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

impl BackendWindow for Window {
  fn id(&self) -> WindowId {
    WindowId::from_raw(self.hwnd.as_ptr() as usize)
  }

  fn next(&self, backend: &dyn Backend) -> Option<Event> {
    None
  }

  fn title(&self, backend: &dyn Backend) -> String {
    self.settings.lock().unwrap().title.clone()
  }

  fn size(&self, backend: &dyn Backend) -> Size {
    self.settings.lock().unwrap().size
  }

  fn position(&self, backend: &dyn Backend) -> Position {
    self.settings.lock().unwrap().position.unwrap() // TODO: Handle None case (probably change this entirely)
  }

  fn key(&self, backend: &dyn Backend, keycode: Code) -> KeyState {
    todo!()
  }

  fn mouse(&self, backend: &dyn Backend, button: MouseButton) -> KeyState {
    todo!()
  }

  fn shift_key(&self, backend: &dyn Backend) -> KeyState {
    todo!()
  }

  fn ctrl_key(&self, backend: &dyn Backend) -> KeyState {
    todo!()
  }

  fn alt_key(&self, backend: &dyn Backend) -> KeyState {
    todo!()
  }

  fn super_key(&self, backend: &dyn Backend) -> KeyState {
    todo!()
  }
}

struct Internal;

impl WindowProcedure for Internal {
  fn on_message(&mut self, mut window: WindowHandle, message: &crate::message::Message) -> Option<Response> {
    println!("{window:?} | {message:?}");

    None
  }
}
