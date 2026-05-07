/*
  Taken almost verbatim from winit. Subject to winit's Apache 2.0 license
*/

use ventana_hal::keyboard::Key;

/// Map the raw X11-style keycode to the `KeyCode` enum.
///
/// X11-style keycodes are offset by 8 from the keycodes the Linux kernel uses.
pub fn raw_keycode_to_physicalkey(keycode: u32) -> Key {
  // scancode_to_physicalkey(keycode.saturating_sub(8))
  todo!()
}
