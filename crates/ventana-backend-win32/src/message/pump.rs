use std::ops::RangeInclusive;

use crate::{
  GetMessageResult, PeekMessageResult, flag::PeekMessageFlags, get_message, handle::window::WindowId, peek_message,
};

use super::thread::ThreadMessage;

pub struct Wait;
pub struct Poll;

pub enum PollingMode {
  Wait,
  Poll,
}

pub struct MessagePump {
  mode: PollingMode,
  hwnd: Option<WindowId>,
  filter: Option<RangeInclusive<u32>>,
  flags: PeekMessageFlags,
  translate: bool,
}

impl Default for MessagePump {
  fn default() -> Self {
    Self {
      mode: PollingMode::Wait,
      hwnd: None,
      filter: None,
      flags: PeekMessageFlags::Remove,
      translate: true,
    }
  }
}

impl MessagePump {
  pub fn with_mode(&mut self, strat: PollingMode) -> &mut Self {
    self.mode = strat;
    self
  }

  // pub fn with_window(&mut self, window: WindowId) -> &mut Self {
  //   self.hwnd = Some(window);
  //   self
  // }

  pub fn with_filter(&mut self, filter: RangeInclusive<u32>) -> &mut Self {
    self.filter = Some(filter);
    self
  }

  pub fn with_translation(&mut self, enable: bool) -> &mut Self {
    self.translate = enable;
    self
  }

  pub fn run(&self) {
    match self.mode {
      PollingMode::Wait => while self.wait_once() {},
      PollingMode::Poll => while self.poll_once() {},
    }
  }

  pub fn run_once(&self) -> bool {
    match self.mode {
      PollingMode::Wait => self.wait_once(),
      PollingMode::Poll => self.poll_once(),
    }
  }

  /// returns false when Quit message is sent
  fn wait_once(&self) -> bool {
    match get_message(self.hwnd, &self.filter) {
      GetMessageResult::Message(msg) => {
        if self.translate {
          msg.translate();
        }
        msg.dispatch();
      }
      GetMessageResult::Quit => return false,
      GetMessageResult::Error(e) => eprintln!("{e}"),
    }

    true
  }

  /// returns false when Quit message is sent
  fn poll_once(&self) -> bool {
    match peek_message(self.hwnd, &self.filter, self.flags) {
      PeekMessageResult::Message(msg) => {
        if self.translate {
          msg.translate();
        }
        msg.dispatch();
      }
      PeekMessageResult::Quit => return false,
      PeekMessageResult::None => (),
    }

    true
  }

  pub fn for_each(&self, f: impl FnMut(Option<ThreadMessage>)) {
    match self.mode {
      PollingMode::Wait => self.wait_for_each(f),
      PollingMode::Poll => self.poll_for_each(f),
    }
  }

  fn wait_for_each(&self, mut f: impl FnMut(Option<ThreadMessage>)) {
    loop {
      match get_message(self.hwnd, &self.filter) {
        GetMessageResult::Message(msg) => {
          if self.translate {
            msg.translate();
          }
          msg.dispatch();
          f(Some(msg));
        }
        GetMessageResult::Quit => break,
        GetMessageResult::Error(e) => eprintln!("{e}"),
      }
    }
  }

  fn poll_for_each(&self, mut f: impl FnMut(Option<ThreadMessage>)) {
    loop {
      match peek_message(self.hwnd, &self.filter, self.flags) {
        PeekMessageResult::Message(msg) => {
          if self.translate {
            msg.translate();
          }
          msg.dispatch();
          f(Some(msg));
        }
        PeekMessageResult::Quit => break,
        PeekMessageResult::None => f(None),
      }
    }
  }
}
