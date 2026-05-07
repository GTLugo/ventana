use {
  super::{
    ThreadPtr,
    XKBH,
    XKBXH,
    byte_slice_to_smol_str,
    compose::{
      ComposeStatus,
      XkbComposeState,
      XkbComposeTable,
    },
    keymap::{
      self,
      XkbKeymap,
    },
    state::XkbState,
  },
  smol_str::SmolStr,
  std::{
    ops::Deref,
    sync::atomic::{
      AtomicBool,
      Ordering,
    },
  },
  ventana_hal::{
    event::KeyEvent,
    input::key::LogicalKey,
    keyboard::{
      KeyState,
      Location,
      Modifiers,
    },
  },
  xkbcommon_dl::{
    self as xkb,
    x11::xcb_connection_t,
    xkb_compose_status,
    xkb_context,
    xkb_context_flags,
  },
};

static RESET_DEAD_KEYS: AtomicBool = AtomicBool::new(false);

#[inline(always)]
pub fn reset_dead_keys() {
  RESET_DEAD_KEYS.store(true, Ordering::SeqCst);
}

#[derive(Debug)]
pub struct Context {
  // NOTE: field order matters.
  pub core_keyboard_id: i32,
  state: Option<XkbState>,
  keymap: Option<XkbKeymap>,
  compose_state1: Option<XkbComposeState>,
  compose_state2: Option<XkbComposeState>,
  _compose_table: Option<XkbComposeTable>,
  context: XkbContext,
  scratch_buffer: Vec<u8>,
}

impl Context {
  pub fn new() -> Option<Self> {
    xkb::xkbcommon_option()?;

    let context = XkbContext::new()?;
    let mut compose_table = XkbComposeTable::new(&context);
    let mut compose_state1 = compose_table.as_ref().and_then(|table| table.new_state());
    let mut compose_state2 = compose_table.as_ref().and_then(|table| table.new_state());

    // Disable compose if anything compose related failed to initialize.
    if compose_table.is_none() || compose_state1.is_none() || compose_state2.is_none() {
      compose_state2 = None;
      compose_state1 = None;
      compose_table = None;
    }

    Some(Self {
      state: None,
      keymap: None,
      compose_state1,
      compose_state2,
      core_keyboard_id: 0,
      _compose_table: compose_table,
      context,
      scratch_buffer: Vec::with_capacity(8),
    })
  }

  pub fn from_x11_xkb(xcb: *mut xcb_connection_t) -> Option<Self> {
    let result = unsafe {
      (XKBXH.xkb_x11_setup_xkb_extension)(
        xcb,
        1,
        2,
        xkbcommon_dl::x11::xkb_x11_setup_xkb_extension_flags::XKB_X11_SETUP_XKB_EXTENSION_NO_FLAGS,
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
        std::ptr::null_mut(),
      )
    };

    if result != 1 {
      return None;
    }

    let mut this = Self::new()?;
    this.core_keyboard_id = unsafe { (XKBXH.xkb_x11_get_core_keyboard_device_id)(xcb) };
    this.set_keymap_from_x11(xcb);
    Some(this)
  }

  pub fn state_mut(&mut self) -> Option<&mut XkbState> {
    self.state.as_mut()
  }

  pub fn keymap_mut(&mut self) -> Option<&mut XkbKeymap> {
    self.keymap.as_mut()
  }

  pub fn set_keymap_from_x11(&mut self, xcb: *mut xcb_connection_t) {
    let keymap = XkbKeymap::from_x11_keymap(&self.context, xcb, self.core_keyboard_id);
    let state = keymap.as_ref().and_then(|keymap| XkbState::new_x11(xcb, keymap));
    if keymap.is_none() || state.is_none() {
      log::warn!("failed to update xkb keymap");
    }
    self.state = state;
    self.keymap = keymap;
  }

  /// Key builder context with the user provided xkb state.
  pub fn key_context(&mut self) -> Option<KeyContext<'_>> {
    let state = self.state.as_mut()?;
    let keymap = self.keymap.as_mut()?;
    let compose_state1 = self.compose_state1.as_mut();
    let compose_state2 = self.compose_state2.as_mut();
    let scratch_buffer = &mut self.scratch_buffer;
    Some(KeyContext { state, keymap, compose_state1, compose_state2, scratch_buffer })
  }

  /// Key builder context with the user provided xkb state.
  ///
  /// Should be used when the original context must not be altered.
  pub fn key_context_with_state<'a>(&'a mut self, state: &'a mut XkbState) -> Option<KeyContext<'a>> {
    let keymap = self.keymap.as_mut()?;
    let compose_state1 = self.compose_state1.as_mut();
    let compose_state2 = self.compose_state2.as_mut();
    let scratch_buffer = &mut self.scratch_buffer;
    Some(KeyContext { state, keymap, compose_state1, compose_state2, scratch_buffer })
  }
}

#[derive(Debug)]
pub struct KeyContext<'a> {
  pub state: &'a mut XkbState,
  pub keymap: &'a mut XkbKeymap,
  compose_state1: Option<&'a mut XkbComposeState>,
  compose_state2: Option<&'a mut XkbComposeState>,
  scratch_buffer: &'a mut Vec<u8>,
}

impl KeyContext<'_> {
  pub fn process_key_event(&mut self, keycode: u32, state: KeyState, repeat: bool) -> KeyEvent {
    let mut event = KeyEventResults::new(self, keycode, !repeat && state == KeyState::Down);
    let code = keymap::raw_keycode_to_physicalkey(keycode);
    let (key, location) = event.key();
    // let text = event.text();
    // let (key_without_modifiers, _) = event.key_without_modifiers();
    // let text_with_all_modifiers = event.text_with_all_modifiers();

    // TODO: Fix modifiers
    KeyEvent { state, key, code, location, modifiers: Modifiers::empty(), repeat }
  }

  fn keysym_to_utf8_raw(&mut self, keysym: u32) -> Option<SmolStr> {
    self.scratch_buffer.clear();
    self.scratch_buffer.reserve(8);
    loop {
      let bytes_written = unsafe {
        (XKBH.xkb_keysym_to_utf8)(
          keysym,
          self.scratch_buffer.as_mut_ptr().cast(),
          self.scratch_buffer.capacity(),
        )
      };
      if bytes_written == 0 {
        return None;
      } else if bytes_written == -1 {
        self.scratch_buffer.reserve(8);
      } else {
        unsafe { self.scratch_buffer.set_len(bytes_written.try_into().unwrap()) };
        break;
      }
    }

    // Remove the null-terminator
    self.scratch_buffer.pop();
    byte_slice_to_smol_str(self.scratch_buffer)
  }
}

struct KeyEventResults<'a, 'b> {
  context: &'a mut KeyContext<'b>,
  keycode: u32,
  keysym: u32,
  compose: ComposeStatus,
}

impl<'a, 'b> KeyEventResults<'a, 'b> {
  fn new(context: &'a mut KeyContext<'b>, keycode: u32, compose: bool) -> Self {
    let keysym = context.state.get_one_sym_raw(keycode);

    let compose = if let Some(state) = context.compose_state1.as_mut().filter(|_| compose) {
      if RESET_DEAD_KEYS.swap(false, Ordering::SeqCst) {
        state.reset();
        context.compose_state2.as_mut().unwrap().reset();
      }
      state.feed(keysym)
    } else {
      ComposeStatus::None
    };

    KeyEventResults { context, keycode, keysym, compose }
  }

  pub fn key(&mut self) -> (LogicalKey, Location) {
    let (key, location) = match self.keysym_to_key(self.keysym) {
      Ok(known) => return known,
      Err(undefined) => undefined,
    };

    if let ComposeStatus::Accepted(xkb_compose_status::XKB_COMPOSE_COMPOSING) = self.compose {
      let compose_state = self.context.compose_state2.as_mut().unwrap();
      // When pressing a dead key twice, the non-combining variant of that character will
      // be produced. Since this function only concerns itself with a single keypress, we
      // simulate this double press here by feeding the keysym to the compose state
      // twice.

      compose_state.feed(self.keysym);
      if matches!(compose_state.feed(self.keysym), ComposeStatus::Accepted(_)) {
        // Extracting only a single `char` here *should* be fine, assuming that no
        // dead key's non-combining variant ever occupies more than one `char`.
        let text = compose_state.get_string(self.context.scratch_buffer);
        let key = LogicalKey::Dead(text.and_then(|s| s.chars().next()));
        (key, location)
      } else {
        (key, location)
      }
    } else {
      let key = self
        .composed_text()
        .unwrap_or_else(|_| self.context.keysym_to_utf8_raw(self.keysym))
        .map(LogicalKey::Character)
        .unwrap_or(key);
      (key, location)
    }
  }

  pub fn key_without_modifiers(&mut self) -> (LogicalKey, Location) {
    // This will become a pointer to an array which libxkbcommon owns, so we don't need to
    // deallocate it.
    let layout = self.context.state.layout(self.keycode);
    let keysym = self.context.keymap.first_keysym_by_level(layout, self.keycode);

    match self.keysym_to_key(keysym) {
      Ok((key, location)) => (key, location),
      Err((key, location)) => {
        let key = self.context.keysym_to_utf8_raw(keysym).map(LogicalKey::Character).unwrap_or(key);
        (key, location)
      },
    }
  }

  fn keysym_to_key(&self, keysym: u32) -> Result<(LogicalKey, Location), (LogicalKey, Location)> {
    let location = keymap::keysym_location(keysym);
    let key = keymap::keysym_to_key(keysym);
    if matches!(key, LogicalKey::Unidentified(_)) { Err((key, location)) } else { Ok((key, location)) }
  }

  pub fn text(&mut self) -> Option<SmolStr> {
    self.composed_text().unwrap_or_else(|_| self.context.keysym_to_utf8_raw(self.keysym))
  }

  // The current behaviour makes it so composing a character overrides attempts to input a
  // control character with the `Ctrl` key. We can potentially add a configuration option
  // if someone specifically wants the opposite behaviour.
  pub fn text_with_all_modifiers(&mut self) -> Option<SmolStr> {
    match self.composed_text() {
      Ok(text) => text,
      Err(_) => self.context.state.get_utf8_raw(self.keycode, self.context.scratch_buffer),
    }
  }

  fn composed_text(&mut self) -> Result<Option<SmolStr>, ()> {
    match self.compose {
      ComposeStatus::Accepted(status) => match status {
        xkb_compose_status::XKB_COMPOSE_COMPOSED => {
          let state = self.context.compose_state1.as_mut().unwrap();
          Ok(state.get_string(self.context.scratch_buffer))
        },
        xkb_compose_status::XKB_COMPOSE_COMPOSING | xkb_compose_status::XKB_COMPOSE_CANCELLED => Ok(None),
        xkb_compose_status::XKB_COMPOSE_NOTHING => Err(()),
      },
      _ => Err(()),
    }
  }
}

#[derive(Debug)]
pub struct XkbContext {
  context: ThreadPtr<xkb_context>,
}

impl XkbContext {
  pub fn new() -> Option<Self> {
    let context = unsafe { (XKBH.xkb_context_new)(xkb_context_flags::XKB_CONTEXT_NO_FLAGS) };

    let context = ThreadPtr::new(context)?;

    Some(Self { context })
  }
}

impl Drop for XkbContext {
  fn drop(&mut self) {
    unsafe {
      (XKBH.xkb_context_unref)(self.context.as_ptr());
    }
  }
}

impl Deref for XkbContext {
  type Target = ThreadPtr<xkb_context>;

  fn deref(&self) -> &Self::Target {
    &self.context
  }
}
