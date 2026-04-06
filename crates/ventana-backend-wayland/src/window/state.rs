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
          wl_shm,
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
      pointer::{
        PointerEventKind,
        PointerHandler,
      },
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
    event::{
      Event,
      WindowEvent,
    },
    settings::WindowSettings,
    types::Flow,
    window::WindowId,
  },
};

pub struct SharedState {
  pub id: WindowId,
  pub width: u32,
  pub height: u32,
  pub should_exit: bool,
  pub event: Option<Event>,
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
    settings: WindowSettings,
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

    window.set_title(settings.title);
    window.commit();

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

  fn draw(&mut self, _conn: &Connection, qh: &QueueHandle<Self>) {
    let width = self.state_lock().width;
    let height = self.state_lock().height;
    let stride = self.state_lock().width as i32 * 4;

    let buffer = self.buffer.get_or_insert_with(|| {
      self
        .pool
        .create_buffer(width as i32, height as i32, stride, wl_shm::Format::Argb8888)
        .expect("create buffer")
        .0
    });

    let canvas = match self.pool.canvas(buffer) {
      Some(canvas) => canvas,
      None => {
        // This should be rare, but if the compositor has not released the previous
        // buffer, we need double-buffering.
        let (second_buffer, canvas) = self
          .pool
          .create_buffer(width as i32, height as i32, stride, wl_shm::Format::Argb8888)
          .expect("create buffer");
        *buffer = second_buffer;
        canvas
      },
    };

    // Draw to the window:
    {
      let shift = self.shift.unwrap_or(0);
      canvas.chunks_exact_mut(4).enumerate().for_each(|(index, chunk)| {
        let x = ((index + shift as usize) % width as usize) as u32;
        let y = (index / width as usize) as u32;

        let a = 0xFF;
        let r = u32::min(((width - x) * 0xFF) / width, ((height - y) * 0xFF) / height);
        let g = u32::min((x * 0xFF) / width, ((height - y) * 0xFF) / height);
        let b = u32::min(((width - x) * 0xFF) / width, (y * 0xFF) / height);
        let color = (a << 24) + (r << 16) + (g << 8) + b;

        let array: &mut [u8; 4] = chunk.try_into().unwrap();
        *array = color.to_le_bytes();
      });

      if let Some(shift) = &mut self.shift {
        *shift = (*shift + 1) % width;
      }
    }

    // Damage the entire window
    self
      .window
      .wl_surface()
      .damage_buffer(0, 0, width as i32, height as i32);

    // Request our next frame
    self.window.wl_surface().frame(qh, self.window.wl_surface().clone());

    // Attach and commit to present.
    buffer.attach_to(self.window.wl_surface()).expect("buffer attach");
    self.window.commit();
  }

  fn send_event(&mut self, event: Event) {
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
    if capability == Capability::Keyboard && self.keyboard.is_some() {
      log::trace!("Unset keyboard capability");
      self.keyboard.take().unwrap().release();
    }

    if capability == Capability::Pointer && self.pointer.is_some() {
      log::trace!("Unset pointer capability");
      self.pointer.take().unwrap().release();
    }
  }

  fn remove_seat(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    seat: sctk::reexports::client::protocol::wl_seat::WlSeat,
  ) {
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
    if self.window.wl_surface() == surface {
      log::trace!("Keyboard focus on window with pressed syms: {keysyms:?}");
      self.keyboard_focus = true;
    }
  }

  fn leave(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    surface: &sctk::reexports::client::protocol::wl_surface::WlSurface,
    serial: u32,
  ) {
    if self.window.wl_surface() == surface {
      log::trace!("Release keyboard focus on window");
      self.keyboard_focus = false;
    }
  }

  fn press_key(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    serial: u32,
    event: sctk::seat::keyboard::KeyEvent,
  ) {
    log::trace!("Key press: {event:?}");
  }

  fn repeat_key(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    serial: u32,
    event: sctk::seat::keyboard::KeyEvent,
  ) {
    log::trace!("Key repeat: {event:?}");
  }

  fn release_key(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    keyboard: &sctk::reexports::client::protocol::wl_keyboard::WlKeyboard,
    serial: u32,
    event: sctk::seat::keyboard::KeyEvent,
  ) {
    log::trace!("Key release: {event:?}");
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
    log::trace!("Update modifiers: {modifiers:?}");
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
    use PointerEventKind::*;
    for event in events {
      // Ignore events for other surfaces
      if &event.surface != self.window.wl_surface() {
        continue;
      }

      match event.kind {
        Enter { .. } => {
          log::trace!("Pointer entered @{:?}", event.position);
        },
        Leave { .. } => {
          log::trace!("Pointer left");
        },
        Motion { .. } => {},
        Press { button, .. } => {
          log::trace!("Press {:x} @ {:?}", button, event.position);
          self.shift = self.shift.xor(Some(0));
        },
        Release { button, .. } => {
          log::trace!("Release {:x} @ {:?}", button, event.position);
        },
        Axis {
          horizontal, vertical, ..
        } => {
          log::trace!("Scroll H:{horizontal:?}, V:{vertical:?}");
        },
      }
    }
  }
}

impl WindowHandler for WaylandState {
  fn request_close(&mut self, conn: &Connection, qh: &QueueHandle<Self>, window: &Window) {
    self.send_event(Event::Window(WindowEvent::CloseRequest));
  }

  fn configure(
    &mut self,
    conn: &Connection,
    qh: &QueueHandle<Self>,
    window: &Window,
    configure: sctk::shell::xdg::window::WindowConfigure,
    serial: u32,
  ) {
    log::trace!("Window configured to: {:?}", configure);

    self.buffer = None;
    self.state_lock().width = configure.new_size.0.map(|v| v.get()).unwrap_or(800);
    self.state_lock().height = configure.new_size.1.map(|v| v.get()).unwrap_or(500);

    // Initiate the first draw.
    if self.first_configure {
      self.first_configure = false;
      self.draw(conn, qh);
    }
  }
}

impl ActivationHandler for WaylandState {
  type RequestData = RequestData;

  fn new_token(&mut self, token: String, data: &Self::RequestData) {
    self
      .xdg_activation
      .as_ref()
      .unwrap()
      .activate::<Self>(self.window.wl_surface(), token);
  }
}

impl ProvidesRegistryState for WaylandState {
  sctk::registry_handlers![OutputState, SeatState];

  fn registry(&mut self) -> &mut RegistryState {
    &mut self.registry_state
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
