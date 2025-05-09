#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
#[repr(u16)]
pub enum MouseButton {
  Unknown = 0,
  Left = 1,
  Right = 2,
  Middle = 3,
  Back = 4,
  Forward = 5,
}

impl MouseButton {
  pub(crate) fn from_state(id: usize) -> MouseButton {
    match id {
      0 => Self::Left,
      1 => Self::Right,
      2 => Self::Middle,
      3 => Self::Back,
      4 => Self::Forward,
      _ => Self::Unknown,
    }
  }
}
