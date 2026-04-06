use {
  super::WaylandWindow,
  ventana_hal::{
    event::Event,
    window::{
      BackendEventIterator,
      BackendWindow,
    },
  },
};

pub struct WaylandEventIterator<'w> {
  window: &'w WaylandWindow,
}

impl<'w> WaylandEventIterator<'w> {
  pub fn new(window: &'w WaylandWindow) -> Self {
    Self { window }
  }
}

impl<'w> BackendEventIterator<'w> for WaylandEventIterator<'w> {
  fn next(&mut self) -> Option<Event> {
    self.window.next_event()
  }
}
