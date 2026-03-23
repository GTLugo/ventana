use {
  crate::window::Window,
  ventana_hal::event::Event,
};

impl Window {
  pub fn iter<'w>(&'w self) -> EventIterator<'w> {
    EventIterator { window: self }
  }

  pub fn iter_mut<'w>(&'w mut self) -> EventIteratorMut<'w> {
    EventIteratorMut { window: self }
  }
}

pub struct EventIterator<'a> {
  window: &'a Window,
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

pub struct EventIteratorMut<'a> {
  window: &'a mut Window,
}

impl<'a> Iterator for EventIteratorMut<'a> {
  type Item = Event;

  fn next(&mut self) -> Option<Self::Item> {
    self.window.next_event()
  }
}

impl<'a> IntoIterator for &'a mut Window {
  type IntoIter = EventIteratorMut<'a>;
  type Item = Event;

  fn into_iter(self) -> Self::IntoIter {
    self.iter_mut()
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
