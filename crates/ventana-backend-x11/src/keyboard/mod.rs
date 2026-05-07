/*!
  Taken almost verbatim from winit. Subject to winit's Apache 2.0 license
*/
#![cfg(linux_platform)]

pub mod compose;
pub mod context;
pub mod keymap;
pub mod state;

use {
  crate::X11,
  smol_str::SmolStr,
  std::{
    ffi::c_char,
    fmt::Debug,
    ptr::NonNull,
    sync::{
      Arc,
      LazyLock,
      RwLock,
      RwLockReadGuard,
      RwLockWriteGuard,
    },
  },
  ventana_hal::{
    event::KeyEvent,
    keyboard::KeyState,
  },
  x11rb::protocol::xproto::{
    KeyButMask,
    Keycode,
  },
  xkbcommon_dl::{
    XkbCommon,
    XkbCommonCompose,
    x11::{
      XkbCommonX11,
      xkbcommon_x11_handle,
    },
    xkbcommon_compose_handle,
    xkbcommon_handle,
  },
};

pub struct ThreadPtr<T>(pub Arc<RwLock<NonNull<T>>>);

unsafe impl<T> Sync for ThreadPtr<T> {}
unsafe impl<T> Send for ThreadPtr<T> {}

impl<T> Debug for ThreadPtr<T>
where
  T: Sized,
{
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.debug_tuple("ThreadPtr").field(&self.0).finish()
  }
}

impl<T> Clone for ThreadPtr<T>
where
  T: Sized,
{
  fn clone(&self) -> Self {
    Self(self.0.clone())
  }
}

impl<T> ThreadPtr<T> {
  pub fn new(t: *mut T) -> Option<Self> {
    Some(Self(Arc::new(RwLock::new(NonNull::new(t)?))))
  }

  pub fn write(&self) -> RwLockWriteGuard<'_, NonNull<T>> {
    self.0.write().unwrap()
  }

  pub fn read(&self) -> RwLockReadGuard<'_, NonNull<T>> {
    self.0.read().unwrap()
  }

  pub fn as_ptr(&self) -> *mut T {
    self.read().as_ptr()
  }
}

static XKBH: LazyLock<&'static XkbCommon> = LazyLock::new(xkbcommon_handle);
static XKBCH: LazyLock<&'static XkbCommonCompose> = LazyLock::new(xkbcommon_compose_handle);
static XKBXH: LazyLock<&'static XkbCommonX11> = LazyLock::new(xkbcommon_x11_handle);

pub fn key_from_x11(keycode: Keycode, state: KeyState) -> Option<KeyEvent> {
  let keycode = keycode as u32;
  let mut context = X11::xkb_context();

  // let keymap = context.keymap_mut()?;

  // // Send the keys using the synthetic state to not alter the main state.
  // let mut xkb_state = XkbState::new_x11(X11::connection().get_raw_xcb_connection(), keymap)?;
  // let mut key_processor = context.key_context_with_state(&mut xkb_state)?;

  // let event = key_processor.process_key_event(keycode as u32, state, false);

  // let code = raw_keycode_to_physicalkey(keycode);
  let key_repeats = context.keymap_mut().map(|k| k.key_repeats(keycode)).unwrap_or(false);

  let repeat = if key_repeats {
    let is_latest_held = *X11::held_key_press() == Some(keycode);

    if state == KeyState::Down {
      *X11::held_key_press() = Some(keycode);
      is_latest_held
    } else {
      // Check that the released key is the latest repeatable key that has been
      // pressed, since repeats will continue for the latest key press if a
      // different previously pressed key is released.
      if is_latest_held {
        *X11::held_key_press() = None;
      }
      false
    }
  } else {
    false
  };

  // // NOTE: When the modifier was captured by the XFilterEvents the modifiers for the modifier
  // // itself are out of sync due to XkbState being delivered before XKeyEvent, since it's
  // // being replayed by the XIM, thus we should replay ourselves.
  // let replay =
  //   if let Some(position) = self.xfiltered_modifiers.iter().rev().position(|&s| s == xev.keycode as u8) {
  //     // We don't have to replay modifiers pressed before the current event if some events
  //     // were not forwarded to us, since their state is irrelevant.
  //     self.xfiltered_modifiers.resize(self.xfiltered_modifiers.len() - 1 - position, 0);
  //     true
  //   } else {
  //     false
  //   };

  let key_processor = context.key_context();

  // log::debug!("{key_processor:?}");

  let mut key_processor = key_processor?;

  let event = key_processor.process_key_event(keycode, state, repeat);

  Some(event)
}

/// Shared logic for constructing a string with `xkb_compose_state_get_utf8` and
/// `xkb_state_key_get_utf8`.
fn make_string_with<F>(scratch_buffer: &mut Vec<u8>, mut f: F) -> Option<SmolStr>
where
  F: FnMut(*mut c_char, usize) -> i32,
{
  let size = f(std::ptr::null_mut(), 0);
  if size == 0 {
    return None;
  }
  let size = usize::try_from(size).unwrap();
  scratch_buffer.clear();
  // The allocated buffer must include space for the null-terminator.
  scratch_buffer.reserve(size + 1);
  unsafe {
    let written = f(scratch_buffer.as_mut_ptr().cast(), scratch_buffer.capacity());
    if usize::try_from(written).unwrap() != size {
      // This will likely never happen.
      return None;
    }
    scratch_buffer.set_len(size);
  };

  byte_slice_to_smol_str(scratch_buffer)
}

// NOTE: This is track_caller so we can have more informative line numbers when logging
#[track_caller]
fn byte_slice_to_smol_str(bytes: &[u8]) -> Option<SmolStr> {
  std::str::from_utf8(bytes)
    .map(SmolStr::new)
    .map_err(|e| log::warn!("UTF-8 received from libxkbcommon ({:?}) was invalid: {e}", bytes))
    .ok()
}
