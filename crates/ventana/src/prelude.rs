#[cfg(feature = "auto-backend")]
pub use backend::AutoBackend;
pub use {
  crate::window::{
    Window,
    WindowOptions,
  },
  hal::{
    backend::Backend as _,
    dpi::{
      LogicalPosition,
      LogicalSize,
      PhysicalPosition,
      PhysicalSize,
      Position,
      Size,
    },
    event::{
      Event,
      WindowEvent,
    },
    settings::WindowSettings,
    types::*,
  },
};
