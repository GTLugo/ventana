#![cfg(target_os = "macos")]

use crate::MacOS;

impl MacOS {
  pub fn new() -> Option<Self> {
    Some(Self)
  }
}
