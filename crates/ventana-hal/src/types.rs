use strum::Display;

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Fullscreen {
  // Exclusive, // todo
  Borderless,
}

#[derive(Default, Debug, Display, Copy, Clone, PartialEq, Eq, Hash)]
pub enum CursorMode {
  #[default]
  Normal,
  Confined,
}

/// The wait behaviour of the window.
#[derive(Default, Debug, Display, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Flow {
  #[default]
  Wait,
  Poll,
}

#[derive(Default, Debug, Display, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Visibility {
  #[default]
  Shown,
  Hidden,
}

#[derive(Default, Debug, Display, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Theme {
  #[default]
  Auto,
  Dark,
  Light,
}

#[derive(Debug, Display, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Focus {
  Gained,
  Lost,
}
