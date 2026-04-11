use {
  std::sync::atomic::{
    AtomicU64,
    Ordering,
  },
  ventana_hal::{
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
  },
};

#[derive(Debug, Clone)]
pub struct CommandEnvelope {
  pub id: CommandId,
  pub command: Command,
}

impl From<Command> for CommandEnvelope {
  fn from(command: Command) -> Self {
    Self {
      id: CommandId::next(),
      command,
    }
  }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct CommandId(u64);

impl CommandId {
  pub fn next() -> Self {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    Self(COUNTER.fetch_add(1, Ordering::Relaxed))
  }
}

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
  Empty,
  GetWindowText(String),
}
