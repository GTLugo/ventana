use {
  super::X11Window,
  ventana_hal::{
    event::Event,
    window::{
      BackendEventIterator,
      BackendWindow,
    },
  },
};

pub struct X11EventIterator<'w> {
  window: &'w X11Window,
}

impl<'w> X11EventIterator<'w> {
  pub fn new(window: &'w X11Window) -> Self {
    Self { window }
  }
}

impl<'w> BackendEventIterator<'w> for X11EventIterator<'w> {
  fn next(&mut self) -> Option<Event> {
    self.window.next_event()
  }
}
