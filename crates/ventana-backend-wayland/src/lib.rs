/*

NOTE: Smithay Client Toolkit is nice, but the build script for some reason can't find the xkbcommon library.
      This is causing the entire crate to fail to build, but Winit seems to have a workaround for this by inmplementing
      certain functions themselves. Despite this, perhaps I should just use wayland-client directly and avoid SCTK altogether?
      The major downside to this is obviously the insane amount of boilerplate I will need to rewrite to satisfy Wayland.

*/

mod backend;
mod event;
mod window;

#[allow(unused)]
use {
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
pub struct Wayland(#[cfg(linux_platform)] Arc<self::backend::WaylandConnection>);

impl Backend for Wayland {
  #[cfg(linux_platform)]
  fn instance() -> Option<&'static Self>
  where
    Self: Sized,
  {
    static INSTANCE: LazyLock<Option<Wayland>> = LazyLock::new(Wayland::new);
    INSTANCE.as_ref()
  }

  #[cfg(linux_platform)]
  fn is_available() -> bool
  where
    Self: Sized,
  {
    wayland_client::Connection::connect_to_env().is_ok()
  }

  fn name(&self) -> &'static str {
    "Wayland"
  }

  #[cfg(linux_platform)]
  fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
    Ok(Arc::new(self::window::WaylandWindow::new(settings)?))
  }

  #[cfg(linux_platform)]
  fn list_available_monitors(&self) -> Result<VecDeque<Arc<dyn BackendMonitor>>, RequestError> {
    todo!()
  }

  #[cfg(linux_platform)]
  fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
    todo!()
  }
}
