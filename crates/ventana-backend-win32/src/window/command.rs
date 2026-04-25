use {
  super::state::SharedInternal,
  std::{
    fmt::Debug,
    sync::Arc,
  },
  ventana_hal::{
    cursor_icon::CursorIcon,
    dpi::{
      Position,
      Size,
    },
    settings::WindowSettings,
    types::{
      CursorMode,
      Fullscreen,
      Visibility,
    },
  },
  win64::user::Window,
};

#[derive(Clone, Debug)]
pub struct CreateInfo {
  pub shared: Arc<SharedInternal>,
  pub settings: WindowSettings,
}

#[allow(unused)]
#[derive(Clone, Debug)]
pub enum Command {
  Empty,
  CreateWindow(CreateInfo),
  // Destroy,
  Redraw,
  GetWindowText,
  SetVisibility(Visibility),
  SetDecorations(Visibility),
  SetWindowText(String),
  SetSize(Size),
  SetPosition(Position),
  SetFullscreen(Option<Fullscreen>),
  SetCursorIcon(CursorIcon),
  SetCursorMode(CursorMode),
  SetCursorVisibility(Visibility),
}

impl Command {}

#[derive(Debug, Clone, PartialEq)]
pub enum CommandResponse {
  Success,
  GetWindowText(String),
  CreateWindow(Window),
}
