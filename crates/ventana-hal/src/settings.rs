use dpi::{Position, Size};
use crate::types::Visibility;

#[derive(Debug, Clone)]
pub struct WindowSettings {
  pub title: String, // Should probably be smol-str
  pub size: Size, // Maybe should make this optional and have backend handle None case
  pub position: Option<Position>,
  pub visibility: Visibility,
}

impl Default for WindowSettings {
  fn default() -> Self {
    Self {
      title: String::from("Window"),
      size: Size::Logical((800.0, 500.0).into()),
      position: None,
      visibility: Default::default(),
    }
  }
}

impl WindowSettings {}
