use crate::{position::Position, size::Size};

#[derive(Debug, Clone)]
pub struct WindowSettings {
  pub title: String,
  pub size: Option<Size>,
  pub position: Option<Position>,
}

impl Default for WindowSettings {
  fn default() -> Self {
    Self {
      title: String::from("Window"),
      size: None,
      position: None,
    }
  }
}

impl WindowSettings {}
