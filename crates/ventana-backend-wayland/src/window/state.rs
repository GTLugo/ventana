use {
  sctk::{
    activation::{
      ActivationHandler,
      ActivationState,
      RequestData,
    },
    compositor::{
      CompositorHandler,
      CompositorState,
    },
    output::{
      OutputHandler,
      OutputState,
    },
    reexports::{
      calloop::LoopHandle,
      client::{
        Connection,
        Proxy,
        QueueHandle,
        globals::GlobalList,
        protocol::{
          wl_keyboard::WlKeyboard,
          wl_pointer::WlPointer,
        },
      },
    },
    registry::{
      ProvidesRegistryState,
      RegistryState,
    },
    seat::{
      Capability,
      SeatHandler,
      SeatState,
      keyboard::KeyboardHandler,
      pointer::PointerHandler,
    },
    shell::{
      WaylandSurface,
      xdg::{
        XdgShell,
        window::{
          Window,
          WindowDecorations,
          WindowHandler,
        },
      },
    },
    shm::{
      Shm,
      ShmHandler,
      slot::{
        Buffer,
        SlotPool,
      },
    },
  },
  std::sync::{
    Arc,
    Mutex,
    MutexGuard,
  },
  synchronize::Signal,
  ventana_hal::{
    error::{
      MapToOSError,
      RequestError,
    },
    types::Flow,
    window::WindowId,
  },
};

pub struct SharedState {
  pub id: WindowId,
  pub width: u32,
  pub height: u32,
  pub should_exit: bool,
  pub event: Option<ventana_hal::event::Event>,
  pub event_signal: Signal,
  pub iteration_signal: Signal,

  pub flow: Flow,
  pub close_on_x: bool,
}

pub struct WaylandState {
  event_signal: Signal,
  iteration_signal: Signal,
  window_state: Arc<Mutex<SharedState>>,
  registry_state: RegistryState,
  seat_state: SeatState,
  output_state: OutputState,
  compositor_state: Arc<CompositorState>,
  shm: Shm,
  shell: XdgShell,
  id: WindowId,
  window: Window,
  xdg_activation: Option<ActivationState>,
  first_configure: bool,
  pool: SlotPool,
  shift: Option<u32>,
  buffer: Option<Buffer>,
  keyboard: Option<WlKeyboard>,
  keyboard_focus: bool,
  pointer: Option<WlPointer>,
  loop_handle: LoopHandle<'static, WaylandState>,
}

impl WaylandState {
  pub fn new(
    globals: &GlobalList,
    queue_handle: &QueueHandle<Self>,
    window_state: Arc<Mutex<SharedState>>,
    loop_handle: LoopHandle<'static, WaylandState>,
  ) -> Result<Self, RequestError> {
    let event_signal = window_state.lock().unwrap().event_signal.clone();
    let iteration_signal = window_state.lock().unwrap().iteration_signal.clone();

    let registry_state = RegistryState::new(globals);
    let seat_state = SeatState::new(globals, queue_handle);
    let output_state = OutputState::new(globals, queue_handle);

    let compositor_state = Arc::new(CompositorState::bind(globals, queue_handle).map_to_os_err()?);
    let shell = XdgShell::bind(globals, queue_handle).map_to_os_err()?;
    let surface = compositor_state.create_surface(queue_handle);
    let id = WindowId::from_raw(surface.id().protocol_id() as usize);
    window_state.lock().unwrap().id = id;
    let shm = Shm::bind(globals, queue_handle).map_to_os_err()?;
    let window = shell.create_window(surface, WindowDecorations::RequestServer, queue_handle);

    let xdg_activation = ActivationState::bind(globals, queue_handle).ok();

    let pool = SlotPool::new(256 * 256 * 4, &shm).map_to_os_err()?;

    Ok(Self {
      event_signal,
      iteration_signal,
      window_state,
      registry_state,
      seat_state,
      output_state,
      compositor_state,
      shm,
      shell,
      id,
      window,
      xdg_activation,
      first_configure: true,
      pool,
      shift: None,
      buffer: None,
      keyboard: None,
      keyboard_focus: false,
      pointer: None,
      loop_handle,
    })
  }

  pub fn state(&self) -> Arc<Mutex<SharedState>> {
    self.window_state.clone()
  }

  pub fn state_lock(&self) -> MutexGuard<'_, SharedState> {
    self.window_state.lock().unwrap()
  }

  pub fn id(&self) -> WindowId {
    self.id
  }

  pub fn window(&self) -> Window {
    self.window.clone()
  }

  pub fn commit(&self) {
    self.window.commit();
  }

  fn draw(&mut self, _conn: &Connection, qh: &QueueHandle<Self>) {}

  fn send_event(&mut self, event: ventana_hal::event::Event) {
    let should_wait = self.state_lock().event.is_some();
    if should_wait {
      self.iteration_signal.wait().unwrap();
    }

    self.state_lock().event.replace(event);
    self.event_signal.signal().unwrap();
    self.iteration_signal.wait().unwrap();
  }
}

impl CompositorHandler for WaylandState {
  fn scale_factor_changed(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    surface: &sctk::reexports::client::protocol::wl_surface::WlSurface,
    new_factor: i32,
  ) {
  }

  fn transform_changed(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    surface: &sctk::reexports::client::protocol::wl_surface::WlSurface,
    new_transform: sctk::reexports::client::protocol::wl_output::Transform,
  ) {
  }

  fn frame(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    surface: &sctk::reexports::client::protocol::wl_surface::WlSurface,
    time: u32,
  ) {
    self.draw(conn, qh);
  }

  fn surface_enter(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    surface: &sctk::reexports::client::protocol::wl_surface::WlSurface,
    output: &sctk::reexports::client::protocol::wl_output::WlOutput,
  ) {
  }

  fn surface_leave(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    surface: &sctk::reexports::client::protocol::wl_surface::WlSurface,
    output: &sctk::reexports::client::protocol::wl_output::WlOutput,
  ) {
  }
}

impl OutputHandler for WaylandState {
  fn output_state(&mut self) -> &mut OutputState {
    &mut self.output_state
  }

  fn new_output(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    output: sctk::reexports::client::protocol::wl_output::WlOutput,
  ) {
  }

  fn update_output(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    output: sctk::reexports::client::protocol::wl_output::WlOutput,
  ) {
  }

  fn output_destroyed(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    output: sctk::reexports::client::protocol::wl_output::WlOutput,
  ) {
  }
}

impl ShmHandler for WaylandState {
  fn shm_state(&mut self) -> &mut sctk::shm::Shm {
    &mut self.shm
  }
}

impl SeatHandler for WaylandState {
  fn seat_state(&mut self) -> &mut SeatState {
    &mut self.seat_state
  }

  fn new_seat(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: sctk::reexports::client::protocol::wl_seat::WlSeat,
  ) {
  }

  fn new_capability(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: sctk::reexports::client::protocol::wl_seat::WlSeat,
    capability: sctk::seat::Capability,
  ) {
    if capability == Capability::Keyboard && self.keyboard.is_none() {
      let keyboard = self
        .seat_state
        .get_keyboard_with_repeat(
          qh,
          &seat,
          None,
          self.loop_handle.clone(),
          Box::new(|_state, _wl_kbd, event| {
            log::trace!("Repeat: {:?} ", event);
          }),
        )
        .map_to_os_err()
        .unwrap();

      self.keyboard = Some(keyboard);
    }

    if capability == Capability::Pointer && self.pointer.is_none() {
      let pointer = self.seat_state.get_pointer(qh, &seat).map_to_os_err().unwrap();
      self.pointer = Some(pointer);
    }
  }

  fn remove_capability(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: sctk::reexports::client::protocol::wl_seat::WlSeat,
    capability: sctk::seat::Capability,
  ) {
    todo!()
  }

  fn remove_seat(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: sctk::reexports::client::protocol::wl_seat::WlSeat,
  ) {
    todo!()
  }
}

impl KeyboardHandler for WaylandState {
  fn enter(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    surface: &sctk::reexports::client::protocol::wl_surface::WlSurface,
    serial: u32,
    raw: &[u32],
    keysyms: &[sctk::seat::keyboard::Keysym],
  ) {
    todo!()
  }

  fn leave(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    surface: &sctk::reexports::client::protocol::wl_surface::WlSurface,
    serial: u32,
  ) {
    todo!()
  }

  fn press_key(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    serial: u32,
    event: sctk::seat::keyboard::KeyEvent,
  ) {
  }

  fn repeat_key(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    serial: u32,
    event: sctk::seat::keyboard::KeyEvent,
  ) {
    todo!()
  }

  fn release_key(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    serial: u32,
    event: sctk::seat::keyboard::KeyEvent,
  ) {
    todo!()
  }

  fn update_modifiers(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    serial: u32,
    modifiers: sctk::seat::keyboard::Modifiers,
    raw_modifiers: sctk::seat::keyboard::RawModifiers,
    layout: u32,
  ) {
    todo!()
  }
}

impl PointerHandler for WaylandState {
  fn pointer_frame(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    pointer: &sctk::reexports::client::protocol::wl_pointer::WlPointer,
    events: &[sctk::seat::pointer::PointerEvent],
  ) {
    todo!()
  }
}

impl WindowHandler for WaylandState {
  fn request_close(&mut self, conn: &Connection, qh: &QueueHandle<Self>, window: &Window) {
    todo!()
  }

  fn configure(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    window: &Window,
    configure: sctk::shell::xdg::window::WindowConfigure,
    serial: u32,
  ) {
    todo!()
  }
}

impl ActivationHandler for WaylandState {
  type RequestData = RequestData;

  fn new_token(&mut self, token: String, data: &Self::RequestData) {
    todo!()
  }
}

impl ProvidesRegistryState for WaylandState {
  sctk::registry_handlers![OutputState, SeatState];

  fn registry(&mut self) -> &mut RegistryState {
    todo!()
  }
}

sctk::delegate_compositor!(WaylandState);
sctk::delegate_output!(WaylandState);
sctk::delegate_shm!(WaylandState);

sctk::delegate_seat!(WaylandState);
sctk::delegate_keyboard!(WaylandState);
sctk::delegate_pointer!(WaylandState);

sctk::delegate_xdg_shell!(WaylandState);
sctk::delegate_xdg_window!(WaylandState);
sctk::delegate_activation!(WaylandState);

sctk::delegate_registry!(WaylandState);
