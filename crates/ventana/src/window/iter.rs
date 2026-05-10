use {
  crate::window::Window,
  hal::event::Event,
};

impl Window {
  pub fn iter(&'_ self) -> EventIterator<'_> {
    EventIterator { window: self }
  }

  pub fn try_iter(&'_ self) -> PollingEventIterator<'_> {
    PollingEventIterator { window: self }
  }
}

// impl Iterator for Window {
//   type Item = Event;

//   fn next(&mut self) -> Option<Self::Item> {
//     self.next_event()
//   }
// }

/*
  ===========================================================
  Waiting Iterator
  ===========================================================
*/

pub struct EventIterator<'w> {
  window: &'w Window,
}

impl<'a> Iterator for EventIterator<'a> {
  type Item = Event;

  fn next(&mut self) -> Option<Self::Item> {
    self.window.next()
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
    self.window.next()
  }
}

impl IntoIterator for Window {
  type IntoIter = WindowIntoIterator;
  type Item = Event;

  fn into_iter(self) -> Self::IntoIter {
    WindowIntoIterator { window: self }
  }
}

/*
  ===========================================================
  Polling Iterator
  ===========================================================
*/

pub struct PollingEventIterator<'w> {
  window: &'w Window,
}

impl<'a> Iterator for PollingEventIterator<'a> {
  type Item = Event;

  fn next(&mut self) -> Option<Self::Item> {
    self.window.try_next()
  }
}

// impl<'a> IntoIterator for &'a Window {
//   type IntoIter = EventIterator<'a>;
//   type Item = Event;

//   fn into_iter(self) -> Self::IntoIter {
//     self.iter()
//   }
// }

// This is unused for now since the default iteration method should be to wait.
pub struct PollingWindowIntoIterator {
  window: Window,
}

impl Iterator for PollingWindowIntoIterator {
  type Item = Event;

  fn next(&mut self) -> Option<Self::Item> {
    self.window.try_next()
  }
}

// impl IntoIterator for Window {
//   type IntoIter = WindowIntoIterator;
//   type Item = Event;

//   fn into_iter(self) -> Self::IntoIter {
//     WindowIntoIterator { window: self }
//   }
// }
