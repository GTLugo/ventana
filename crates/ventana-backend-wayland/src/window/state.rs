use {
  sctk::{
    reexports::client::{
      Connection,
      Dispatch,
      Proxy,
      QueueHandle,
      globals::{
        GlobalList,
        GlobalListContents,
      },
      protocol::{
        wl_registry::WlRegistry,
        wl_surface::WlSurface,
      },
    },
    registry::RegistryState,
  },
  ventana_hal::error::RequestError,
};

pub struct WaylandWindowState {
  pub registry_state: RegistryState,
}

impl WaylandWindowState {
  pub fn new(globals: &GlobalList, queue_handle: &QueueHandle<Self>) -> Result<Self, RequestError> {
    let registry_state = RegistryState::new(globals);
    Ok(Self { registry_state })
  }
}

impl Dispatch<WlRegistry, GlobalListContents> for WaylandWindowState {
  fn event(
    state: &mut Self,
    proxy: &WlRegistry,
    event: <WlRegistry as Proxy>::Event,
    data: &GlobalListContents,
    connection: &Connection,
    handle: &QueueHandle<Self>,
  ) {
  }
}

impl Dispatch<WlSurface, GlobalListContents> for WaylandWindowState {
  fn event(
    state: &mut Self,
    proxy: &WlSurface,
    event: <WlSurface as Proxy>::Event,
    data: &GlobalListContents,
    connection: &Connection,
    handle: &QueueHandle<Self>,
  ) {
  }
}
