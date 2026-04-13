#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))] // TODO: Swap this out for a stub impl on other platforms.

/*

NOTE: Smithay Client Toolkit is nice, but the build script for some reason can't find the xkbcommon library.
      This is causing the entire crate to fail to build, but Winit seems to have a workaround for this by inmplementing
      certain functions themselves. Despite this, perhaps I should just use wayland-client directly and avoid SCTK altogether?
      The major downside to this is obviously the insane amount of boilerplate I will need to rewrite to satisfy Wayland.

*/

// mod event;
pub mod window;

use {
  self::window::WaylandWindow,
  std::{
    collections::VecDeque,
    sync::{
      Arc,
      LazyLock,
    },
  },
  ventana_hal::{
    backend::Backend,
    error::RequestError,
    monitor::BackendMonitor,
    settings::WindowSettings,
    window::BackendWindow,
  },
};

#[derive(Clone)]
pub struct Wayland;

impl Wayland {
  pub fn connect() -> Result<(), RequestError> {
    todo!()
  }
}

impl Backend for Wayland {
  fn instance() -> Option<&'static Self>
  where
    Self: Sized,
  {
    static INSTANCE: LazyLock<Option<Wayland>> = LazyLock::new(|| Wayland::connect().ok().map(|_| Wayland));
    INSTANCE.as_ref()
  }

  fn is_available() -> bool
  where
    Self: Sized,
  {
    cfg!(all(
      unix,
      not(any(target_os = "redox", target_family = "wasm", target_os = "android", target_vendor = "apple"))
    )) && Wayland::connect().is_ok()
  }

  fn name(&self) -> &'static str {
    "Wayland"
  }

  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(WaylandWindow))
  }

  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    todo!()
  }

  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    todo!()
  }
}
