use {
  crate::{
    event::Event,
    monitor::BackendMonitor,
  },
  dpi::{
    PhysicalPosition,
    PhysicalSize,
  },
  keyboard_types::{
    Code,
    KeyState,
  },
  pointer_types::{
    ButtonState,
    PointerButton,
  },
  raw_window_handle::{
    RawDisplayHandle,
    RawWindowHandle,
  },
  std::sync::Arc,
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct WindowId(usize);

impl WindowId {
  pub const fn to_raw(self) -> usize {
    self.0
  }

  pub const fn from_raw(raw: usize) -> Self {
    Self(raw)
  }
}

impl std::fmt::Display for WindowId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

// Maybe split this into things like "BasicWindow" "ResizableWindow" "MoveableWindow" etc
pub trait BackendWindow: Send + Sync {
  fn id(&self) -> WindowId;

  fn raw_window_handle(&self) -> RawWindowHandle;

  fn raw_display_handle(&self) -> RawDisplayHandle;

  fn next(&self) -> Option<Event>;

  fn monitor(&self) -> Arc<dyn BackendMonitor>;

  fn close(&self);

  fn is_closing(&self) -> bool;

  fn request_redraw(&self);

  fn title(&self) -> String;

  fn set_title(&self, title: String);

  fn scale_factor(&self) -> f64;

  fn inner_size(&self) -> PhysicalSize<u32>;

  fn outer_size(&self) -> PhysicalSize<u32>;

  fn inner_position(&self) -> PhysicalPosition<i32>;

  fn outer_position(&self) -> PhysicalPosition<i32>;

  fn key(&self, code: Code) -> KeyState;

  fn pointer(&self, button: PointerButton) -> ButtonState;

  fn shift_key(&self) -> KeyState;

  fn ctrl_key(&self) -> KeyState;

  fn alt_key(&self) -> KeyState;

  fn meta_key(&self) -> KeyState;
}

// pub trait BackendEventIterator<'window>: Send + Sync {
//   fn next(&mut self) -> Option<Event>;
// }
