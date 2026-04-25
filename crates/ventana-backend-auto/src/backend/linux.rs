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
    #[cfg(feature = "wayland")]
    {
      Wayland::is_available()
    }
    #[cfg(not(feature = "wayland"))]
    false
  }

  pub fn is_x11_available() -> bool {
    #[cfg(feature = "x11")]
    {
      X11::is_available()
    }
    #[cfg(not(feature = "x11"))]
    false
  }

  pub fn is_available() -> bool {
    log::info!("Wayland available: {} | X11 available: {}", Self::is_wayland_available(), Self::is_x11_available());
    Self::is_wayland_available() || Self::is_x11_available()
  }

  pub fn instance() -> Option<&'static dyn Backend> {
    let wayland = {
      #[cfg(feature = "wayland")]
      {
        Wayland::instance()
      }
      #[cfg(not(feature = "wayland"))]
      {
        None
      }
    };

    let x11 = {
      #[cfg(feature = "x11")]
      {
        || X11::instance()
      }
      #[cfg(not(feature = "x11"))]
      {
        || None
      }
    };

    wayland.or_else(x11)
  }
}
