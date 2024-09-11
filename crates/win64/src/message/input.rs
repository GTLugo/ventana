use windows::Win32::UI::WindowsAndMessaging;

use super::{FromMessage, RawMessage};

#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardMessage {}

impl KeyboardMessage {}

impl FromMessage for KeyboardMessage {
  type Err = ();

  fn from_message(msg: &RawMessage) -> Result<Self, Self::Err> {
    match msg.id() {
      WindowsAndMessaging::WM_KEYFIRST..=WindowsAndMessaging::WM_KEYLAST => Ok(Self {}),
      _ => Err(()),
    }
  }
}
