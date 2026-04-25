#[cfg(feature = "wayland")]
pub use backend_wayland::Wayland;
#[cfg(feature = "x11")]
pub use backend_x11::X11;
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
    Self::is_wayland_available() || Self::is_x11_available()
  }

  pub fn instance() -> Option<&'static dyn Backend> {
    log::info!("Wayland available: {}", Self::is_wayland_available());
    let wayland = {
      #[cfg(feature = "wayland")]
      {
        Wayland::instance().map(|b| b as _)
      }
      #[cfg(not(feature = "wayland"))]
      {
        None
      }
    };

    log::info!("X11 available: {}", Self::is_x11_available());
    let x11 = {
      #[cfg(feature = "x11")]
      {
        || X11::instance().map(|b| b as _)
      }
      #[cfg(not(feature = "x11"))]
      {
        || None
      }
    };

    wayland.or_else(x11)
  }
}
