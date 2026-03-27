use {
  crate::window::Window,
  ventana_hal::{
    event::Event,
    window::BackendEventIterator,
  },
};

impl Window {
  pub fn iter<'w>(&'w self) -> EventIterator<'w> {
    EventIterator(self.window.iter())
  }
}

pub struct EventIterator<'w>(Box<dyn BackendEventIterator<'w> + 'w>);

impl<'a> Iterator for EventIterator<'a> {
  type Item = Event;

  fn next(&mut self) -> Option<Self::Item> {
    self.0.next()
  }
}

impl<'a> IntoIterator for &'a Window {
  type IntoIter = EventIterator<'a>;
  type Item = Event;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

/*
  IntoIterator for value type needs more work, but I'm too tired to debug it.

  The issue I had was it immediately closing the window after returning None after first initialization.
*/

// pub struct WindowIntoIterator {
//   window: Window,
// }

// impl Iterator for WindowIntoIterator {
//   type Item = Event;

//   fn next(&mut self) -> Option<Self::Item> {
//     self.window.next_event()
//   }
// }

// impl IntoIterator for Window {
//   type IntoIter = WindowIntoIterator;
//   type Item = Event;

//   fn into_iter(self) -> Self::IntoIter {
//     WindowIntoIterator { window: self }
//   }
// }
