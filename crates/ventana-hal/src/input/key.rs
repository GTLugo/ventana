/*
  This is largely made to match Winit's semantics. As Winit evolves, this will likely
  evolve alongside them to maintain compatibility.
*/

use {
  keyboard_types::{
    Code,
    NamedKey,
  },
  smol_str::SmolStr,
};

/// This is designed to carry any native platform **logical** key identifier.
///
/// This has the inherent tradeoff of being excessively large on platforms with smaller id types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NativeKey {
  Unidentified,
  Key(u32),
}

/// This is designed to carry any native platform **physical** key identifier.
///
/// This has the inherent tradeoff of being excessively large on platforms with smaller id types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum NativeCode {
  Unidentified,
  Code(u32),
}

impl From<NativeCode> for NativeKey {
  #[inline]
  fn from(code: NativeCode) -> Self {
    match code {
      NativeCode::Unidentified => NativeKey::Unidentified,
      NativeCode::Code(x) => NativeKey::Key(x),
    }
  }
}

/// LogicalKey represents the meaning of a keypress.
///
/// This is a superset of the UI Events Specification's [`KeyboardEvent.key`] with
/// additions:
/// - All simple variants are wrapped under the `Named` variant
/// - The `Unidentified` variant here, can still identify a key through it's `NativeKeyCode`.
/// - The `Dead` variant here, can specify the character which is inserted when pressing the
///   dead-key twice.
///
/// [`KeyboardEvent.key`]: https://w3c.github.io/uievents-key/
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum LogicalKey<Str = SmolStr> {
  /// A simple (unparameterised) action
  Named(NamedKey),

  /// A key string that corresponds to the character typed by the user, taking into account the
  /// user’s current locale setting, and any system-level keyboard mapping overrides that are in
  /// effect.
  Character(Str),

  /// This variant is used when the key cannot be translated to any other variant.
  ///
  /// The native key is provided (if available) in order to allow the user to specify keybindings
  /// for keys which are not defined by this API, mainly through some sort of UI.
  Unidentified(NativeKey),

  /// Contains the text representation of the dead-key when available.
  Dead(Option<char>),
}

impl LogicalKey<SmolStr> {
  /// Convert `Key::Character(SmolStr)` to `Key::Character(&str)` so you can more easily match on
  /// `Key`. All other variants remain unchanged.
  pub fn as_ref(&self) -> LogicalKey<&str> {
    match self {
      Self::Named(a) => LogicalKey::Named(*a),
      Self::Character(ch) => LogicalKey::Character(ch.as_str()),
      Self::Dead(d) => LogicalKey::Dead(*d),
      Self::Unidentified(u) => LogicalKey::Unidentified(*u),
    }
  }
}
impl From<NamedKey> for LogicalKey {
  #[inline]
  fn from(action: NamedKey) -> Self {
    Self::Named(action)
  }
}

impl From<NativeKey> for LogicalKey {
  #[inline]
  fn from(code: NativeKey) -> Self {
    Self::Unidentified(code)
  }
}

impl<Str> PartialEq<NamedKey> for LogicalKey<Str> {
  #[inline]
  fn eq(&self, rhs: &NamedKey) -> bool {
    match self {
      Self::Named(a) => a == rhs,
      _ => false,
    }
  }
}

impl<Str: PartialEq<str>> PartialEq<str> for LogicalKey<Str> {
  #[inline]
  fn eq(&self, rhs: &str) -> bool {
    match self {
      Self::Character(s) => s == rhs,
      _ => false,
    }
  }
}

impl<Str: PartialEq<str>> PartialEq<&str> for LogicalKey<Str> {
  #[inline]
  fn eq(&self, rhs: &&str) -> bool {
    self == *rhs
  }
}

impl<Str> PartialEq<NativeKey> for LogicalKey<Str> {
  #[inline]
  fn eq(&self, rhs: &NativeKey) -> bool {
    match self {
      Self::Unidentified(code) => code == rhs,
      _ => false,
    }
  }
}

impl<Str> PartialEq<LogicalKey<Str>> for NativeKey {
  #[inline]
  fn eq(&self, rhs: &LogicalKey<Str>) -> bool {
    rhs == self
  }
}

impl LogicalKey {
  /// Convert a key to its approximate textual equivalent.
  pub fn to_text(&self) -> Option<&str> {
    match self {
      Self::Named(action) => match action {
        NamedKey::Enter => Some("\r"),
        NamedKey::Backspace => Some("\x08"),
        NamedKey::Tab => Some("\t"),
        NamedKey::Escape => Some("\x1b"),
        _ => None,
      },
      Self::Character(ch) => Some(ch.as_str()),
      _ => None,
    }
  }
}

/// Represents the location of a physical key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub enum PhysicalKey {
  /// A known key code
  Code(Code),
  /// This variant is used when the key cannot be translated to a [`KeyCode`]
  ///
  /// The native keycode is provided (if available) so you're able to more reliably match
  /// key-press and key-release events by hashing the [`PhysicalKey`]. It is also possible to use
  /// this for keybinds for non-standard keys, but such keybinds are tied to a given platform.
  Unidentified(NativeCode),
}

impl From<Code> for PhysicalKey {
  #[inline]
  fn from(code: Code) -> Self {
    Self::Code(code)
  }
}

impl From<PhysicalKey> for Code {
  #[inline]
  fn from(key: PhysicalKey) -> Self {
    match key {
      PhysicalKey::Code(code) => code,
      PhysicalKey::Unidentified(_) => Code::Unidentified,
    }
  }
}
impl From<NativeCode> for PhysicalKey {
  #[inline]
  fn from(code: NativeCode) -> Self {
    Self::Unidentified(code)
  }
}

impl PartialEq<Code> for PhysicalKey {
  #[inline]
  fn eq(&self, rhs: &Code) -> bool {
    match self {
      Self::Code(code) => code == rhs,
      _ => false,
    }
  }
}

impl PartialEq<PhysicalKey> for Code {
  #[inline]
  fn eq(&self, rhs: &PhysicalKey) -> bool {
    rhs == self
  }
}

impl PartialEq<NativeCode> for PhysicalKey {
  #[inline]
  fn eq(&self, rhs: &NativeCode) -> bool {
    match self {
      Self::Unidentified(code) => code == rhs,
      _ => false,
    }
  }
}

impl PartialEq<PhysicalKey> for NativeCode {
  #[inline]
  fn eq(&self, rhs: &PhysicalKey) -> bool {
    rhs == self
  }
}
