use {
  crate::types::Visibility,
  dpi::{
    Position,
    Size,
  },
  rgb::RGB8,
  smol_str::SmolStr,
};

#[derive(Debug, Clone)]
pub struct WindowSettings {
  pub title: SmolStr,
  pub size: Size, // Maybe should make this optional and have backend handle None case
  pub position: Option<Position>,
  pub visibility: Visibility,
  // pub flow: Flow,
  pub close_on_x: bool,
  pub clear_color: Option<RGB8>,
  // pub reveal_delay_frames: Option<u32>, // Future feature to offer a way around Win32's infamous White Flash (TM)
}

impl WindowSettings {}
