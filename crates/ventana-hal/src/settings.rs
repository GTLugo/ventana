use {
  crate::types::Visibility,
  dpi::{
    Position,
    Size,
  },
  smol_str::SmolStr,
};

#[derive(Debug, Clone)]
pub struct WindowSettings {
  pub title: SmolStr,
  pub size: Size, // Maybe should make this optional and have backend handle None case
  pub position: Option<Position>,
  pub visibility: Visibility,
}

impl WindowSettings {}
