use std::ops::RangeInclusive;

use crate::{
  flag::PeekMessageFlags, get_message, handle::window::WindowId, peek_message, GetMessageResult, PeekMessageResult,
};

use super::{Metadata, RawMessage};

pub struct Wait;
pub struct Poll;

pub enum PumpStrategy {
  Wait,
  Poll,
}

pub struct MessagePump {
  strat: PumpStrategy,
  hwnd: Option<WindowId>,
  filter: Option<RangeInclusive<u32>>,
  flags: PeekMessageFlags,
  translate: bool,
}

impl Default for MessagePump {
  fn default() -> Self {
    Self {
      strat: PumpStrategy::Wait,
      hwnd: None,
      filter: None,
      flags: PeekMessageFlags::Remove,
      translate: true,
    }
  }
}

impl MessagePump {
  pub fn with_strategy(&mut self, strat: PumpStrategy) -> &mut Self {
    self.strat = strat;
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
    match self.strat {
      PumpStrategy::Wait => while self.wait_once() {},
      PumpStrategy::Poll => while self.poll_once() {},
    }
  }

  pub fn run_once(&self) -> bool {
    match self.strat {
      PumpStrategy::Wait => self.wait_once(),
      PumpStrategy::Poll => self.poll_once(),
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

  pub fn for_each(&self, f: impl FnMut(Option<RawMessage<Metadata>>)) {
    match self.strat {
      PumpStrategy::Wait => self.wait_for_each(f),
      PumpStrategy::Poll => self.poll_for_each(f),
    }
  }

  fn wait_for_each(&self, mut f: impl FnMut(Option<RawMessage<Metadata>>)) {
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

  fn poll_for_each(&self, mut f: impl FnMut(Option<RawMessage<Metadata>>)) {
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
