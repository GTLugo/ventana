use ventana_hal::{
  cursor_icon::CursorIcon,
  dpi::{
    Position,
    Size,
  },
  types::{
    CursorMode,
    Fullscreen,
    Visibility,
  },
};

#[allow(unused)]
#[derive(Debug, Clone, PartialEq)]
pub enum Command {
  Destroy,
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

#[derive(Debug, Clone, PartialEq)]
pub enum CommandResponse {
  Success,
  GetWindowText(String),
}
