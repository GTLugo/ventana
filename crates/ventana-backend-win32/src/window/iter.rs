use {
  super::Win32Window,
  ventana_hal::{
    event::Event,
    window::{
      BackendEventIterator,
      BackendWindow,
    },
  },
};

pub struct Win32EventIterator<'w> {
  window: &'w Win32Window,
}

impl<'w> Win32EventIterator<'w> {
  pub fn new(window: &'w Win32Window) -> Self {
    Self { window }
  }
}

impl<'w> BackendEventIterator<'w> for Win32EventIterator<'w> {
  fn next(&mut self) -> Option<Event> {
    self.window.next_event()
  }
}
