pub mod backend;

/*

  Re-exports

*/
#[cfg(wayland_platform)]
pub use backend_wayland::Wayland;
#[cfg(x11_platform)]
pub use backend_x11::X11;
use hal::backend::Backend;
pub use {
  backend::*,
  backend_win32 as win32,
  win32::Win32,
};

struct Linux;

impl Linux {
  fn is_wayland_available() -> bool {
    #[cfg(wayland_platform)]
    {
      crate::Wayland::is_available()
    }
    #[cfg(not(wayland_platform))]
    false
  }

  fn is_x11_available() -> bool {
    #[cfg(x11_platform)]
    {
      crate::X11::is_available()
    }
    #[cfg(not(x11_platform))]
    false
  }

  fn is_available() -> bool {
    Self::is_wayland_available() || Self::is_x11_available()
  }

  fn instance() -> Option<&'static dyn Backend> {
    log::info!("Wayland available: {}", Self::is_wayland_available());
    let wayland = {
      #[cfg(wayland_platform)]
      {
        crate::Wayland::instance().map(|b| b as _)
      }
      #[cfg(not(wayland_platform))]
      {
        None
      }
    };

    log::info!("X11 available: {}", Self::is_x11_available());
    let x11 = {
      #[cfg(x11_platform)]
      {
        || crate::X11::instance().map(|b| b as _)
      }
      #[cfg(not(x11_platform))]
      {
        || None
      }
    };

    wayland.or_else(x11)
  }
}
