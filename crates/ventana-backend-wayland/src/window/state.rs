use {
  sctk::{
    compositor::{
      CompositorHandler,
      CompositorState,
    },
    output::{
      OutputHandler,
      OutputState,
    },
    registry::{
      ProvidesRegistryState,
      RegistryState,
    },
    seat::{
      SeatHandler,
      SeatState,
    },
    shell::xdg::window::{
      Window,
      WindowConfigure,
      WindowHandler,
    },
    shm::{
      Shm,
      ShmHandler,
    },
    subcompositor::SubcompositorState,
  },
  std::sync::{
    Arc,
    atomic::Ordering,
  },
  ventana_hal::{
    monitor::Monitor,
    window::WindowId,
  },
  wayland_client::{
    Connection,
    QueueHandle,
    protocol::{
      wl_output::WlOutput,
      wl_surface::WlSurface,
    },
  },
};

pub struct WindowState {
  pub registry_state: RegistryState,
  pub output_state: OutputState,
  pub compositor_state: Arc<CompositorState>,
  pub subcompositor_state: Option<Arc<SubcompositorState>>,
  pub seat_state: SeatState,
  pub shm: Shm,
}

impl ShmHandler for WindowState {
  fn shm_state(&mut self) -> &mut Shm {
    &mut self.shm
  }
}

impl WindowHandler for WindowState {
  fn request_close(&mut self, _: &Connection, _: &QueueHandle<Self>, window: &Window) {
    todo!()
  }

  fn configure(
    &mut self,
    _: &Connection,
    _: &QueueHandle<Self>,
    window: &Window,
    configure: WindowConfigure,
    _serial: u32,
  ) {
    todo!()
  }
}

impl OutputHandler for WindowState {
  fn output_state(&mut self) -> &mut OutputState {
    &mut self.output_state
  }

  fn new_output(&mut self, _: &Connection, _: &QueueHandle<Self>, output: WlOutput) {
    todo!()
  }

  fn update_output(&mut self, _: &Connection, _: &QueueHandle<Self>, updated: WlOutput) {
    todo!()
  }

  fn output_destroyed(&mut self, _: &Connection, _: &QueueHandle<Self>, removed: WlOutput) {
    todo!()
  }
}

impl CompositorHandler for WindowState {
  fn transform_changed(
    &mut self,
    _: &Connection,
    _: &QueueHandle<Self>,
    _: &WlSurface,
    _: wayland_client::protocol::wl_output::Transform,
  ) {
    // TODO(kchibisov) we need to expose it somehow in winit.
  }

  fn surface_enter(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlSurface, _: &WlOutput) {}

  fn surface_leave(&mut self, _: &Connection, _: &QueueHandle<Self>, _: &WlSurface, _: &WlOutput) {}

  fn scale_factor_changed(
    &mut self,
    _: &Connection,
    _: &QueueHandle<Self>,
    surface: &WlSurface,
    scale_factor: i32,
  ) {
    todo!()
  }

  fn frame(&mut self, _: &Connection, _: &QueueHandle<Self>, surface: &WlSurface, _: u32) {
    todo!()
  }
}

impl SeatHandler for WindowState {
  fn seat_state(&mut self) -> &mut SeatState {
    todo!()
  }

  fn new_seat(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: wayland_client::protocol::wl_seat::WlSeat,
  ) {
    todo!()
  }

  fn new_capability(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: wayland_client::protocol::wl_seat::WlSeat,
    capability: sctk::seat::Capability,
  ) {
    todo!()
  }

  fn remove_capability(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: wayland_client::protocol::wl_seat::WlSeat,
    capability: sctk::seat::Capability,
  ) {
    todo!()
  }

  fn remove_seat(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: wayland_client::protocol::wl_seat::WlSeat,
  ) {
    todo!()
  }
}

impl ProvidesRegistryState for WindowState {
  sctk::registry_handlers![OutputState, SeatState];

  fn registry(&mut self) -> &mut RegistryState {
    &mut self.registry_state
  }
}

// The window update coming from the compositor.
#[derive(Debug, Clone, Copy)]
pub struct WindowCompositorUpdate {
  /// The id of the window this updates belongs to.
  pub window_id: WindowId,

  /// New window size.
  pub resized: bool,

  /// New scale factor.
  pub scale_changed: bool,

  /// Close the window.
  pub close_window: bool,
}

impl WindowCompositorUpdate {
  fn new(window_id: WindowId) -> Self {
    Self { window_id, resized: false, scale_changed: false, close_window: false }
  }
}

sctk::delegate_subcompositor!(WindowState);
sctk::delegate_compositor!(WindowState);
sctk::delegate_output!(WindowState);
sctk::delegate_registry!(WindowState);
sctk::delegate_seat!(WindowState);
sctk::delegate_shm!(WindowState);
sctk::delegate_xdg_shell!(WindowState);
sctk::delegate_xdg_window!(WindowState);
