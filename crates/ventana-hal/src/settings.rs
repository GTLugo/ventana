use crate::{position::Position, size::Size};

#[derive(Debug, Clone)]
pub struct WindowSettings {
  pub title: String,
  pub size: Size,
  pub position: Position,
}

impl Default for WindowSettings {
  fn default() -> Self {
    Self {
      title: String::from("Window"),
      size: Size::Logical((800.0, 500.0).into()),
      position: Position::Logical((0.0, 0.0).into()),
    }
  }
}

impl WindowSettings {}
