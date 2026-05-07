#[cfg(feature = "wayland")]
pub use backend_wayland::{
  self as wayland,
  Wayland,
};
#[cfg(feature = "x11")]
pub use backend_x11::{
  self as x11,
  X11,
};
use hal::backend::Backend;

pub struct Linux;

impl Linux {
  pub fn is_wayland_available() -> bool {
    cfg_select! {
      feature = "wayland" => Wayland::is_available(),
      _ => false
    }
  }

  pub fn is_x11_available() -> bool {
    cfg_select! {
      feature = "x11" => X11::is_available(),
      _ => false
    }
  }

  pub fn is_available() -> bool {
    log::info!(
      "Wayland available: {} | X11 available: {}",
      Self::is_wayland_available(),
      Self::is_x11_available()
    );
    Self::is_wayland_available() || Self::is_x11_available()
  }

  pub fn wayland() -> Option<&'static dyn Backend> {
    cfg_select! {
      feature = "wayland" => X11::backend(),
      _ => None
    }
  }

  pub fn x11() -> Option<&'static dyn Backend> {
    cfg_select! {
      feature = "x11" => X11::backend(),
      _ => None
    }
  }

  pub fn backend() -> Option<&'static dyn Backend> {
    Self::wayland().or_else(Self::x11)
  }
}
