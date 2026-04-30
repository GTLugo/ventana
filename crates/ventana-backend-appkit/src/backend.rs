#![cfg(target_os = "macos")]

use crate::AppKit;

impl AppKit {
  pub fn new() -> Option<Self> {
    Some(Self)
  }
}
