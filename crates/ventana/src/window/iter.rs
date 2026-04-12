use {
  crate::window::Window,
  ventana_hal::{
    event::Event,
  },
};

impl Window {
  pub fn iter(&'_ self) -> EventIterator<'_> {
    EventIterator { window: self }
  }
}

// impl Iterator for Window {
//   type Item = Event;

//   fn next(&mut self) -> Option<Self::Item> {
//     self.next_event()
//   }
// }

pub struct EventIterator<'w> {
  window: &'w Window,
}

impl<'a> Iterator for EventIterator<'a> {
  type Item = Event;

  fn next(&mut self) -> Option<Self::Item> {
    self.window.next_event()
  }
}

impl<'a> IntoIterator for &'a Window {
  type IntoIter = EventIterator<'a>;
  type Item = Event;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

pub struct WindowIntoIterator {
  window: Window,
}

impl Iterator for WindowIntoIterator {
  type Item = Event;

  fn next(&mut self) -> Option<Self::Item> {
    self.window.next_event()
  }
}

impl IntoIterator for Window {
  type IntoIter = WindowIntoIterator;
  type Item = Event;

  fn into_iter(self) -> Self::IntoIter {
    WindowIntoIterator { window: self }
  }
}
