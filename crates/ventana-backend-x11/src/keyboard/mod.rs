#![cfg(linux_platform)]

mod keymap;

use {
  ventana_hal::{
    event::WindowEvent,
    keyboard::{
      Code,
      Key,
      Location,
      Modifiers,
    },
  },
  x11rb::protocol::{
    Event as X11Event,
    xproto::{
      KeyButMask,
      Keycode,
    },
  },
};

pub fn key_from_x11(keycode: Keycode, mods: KeyButMask) -> WindowEvent {
  todo!()
}
