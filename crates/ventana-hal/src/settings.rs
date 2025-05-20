use dpi::{Position, Size};
use crate::types::Visibility;

#[derive(Debug, Clone)]
pub struct WindowSettings {
  pub title: String, // Should probably be smol-str
  pub size: Size,
  pub position: Position,
  pub visibility: Visibility,
}

impl Default for WindowSettings {
  fn default() -> Self {
    Self {
      title: String::from("Window"),
      size: Size::Logical((800.0, 500.0).into()),
      position: Position::Logical((0.0, 0.0).into()),
      visibility: Default::default(),
    }
  }
}

impl WindowSettings {}
