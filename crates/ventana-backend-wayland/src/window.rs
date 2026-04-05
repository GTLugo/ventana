mod state;

use {
  self::state::{
    SharedState,
    WaylandState,
  },
  sctk::reexports::{
    calloop::{
      EventLoop,
      LoopSignal,
    },
    calloop_wayland_source::WaylandSource,
    client::{
      Connection,
      globals::{
        self,
      },
    },
  },
  std::sync::{
    Arc,
    Mutex,
  },
  ventana_hal::{
    error::{
      MapToOSError,
      RequestError,
    },
    settings::WindowSettings,
    window::{
      BackendWindow,
      WindowId,
    },
  },
};

pub struct CreateInfo {
  pub id: WindowId,
  pub loop_signal: LoopSignal,
}

pub struct WaylandWindow {
  id: WindowId,
  state: Arc<Mutex<SharedState>>,
  loop_signal: LoopSignal,
}

impl WaylandWindow {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    // I can't find any decent beginner materials on how to actually make a Wayland window and read events,
    // so this is HEAVILY based on Winit. If there are any quirks, it's probably because of me
    // trying to warp Winit's implementation.

    let state = Arc::new(Mutex::new(SharedState {
      id: WindowId::from_raw(0),
      width: 0,
      height: 0,
      should_exit: false,
    }));
    let shared_state = state.clone();

    let connection = Connection::connect_to_env().map_to_os_err()?;
    let (globals, event_queue) = globals::registry_queue_init(&connection).map_to_os_err()?;
    let queue_handle = event_queue.handle();

    let (info_tx, info_rx) = std::sync::mpsc::sync_channel(0);

    std::thread::Builder::new()
      .name("window".into())
      .spawn(move || -> Result<(), RequestError> {
        let mut event_loop: EventLoop<WaylandState> = EventLoop::try_new().map_to_os_err()?;

        WaylandSource::new(connection.clone(), event_queue)
          .insert(event_loop.handle())
          .map_to_os_err()?;

        let mut wayland_state = WaylandState::new(&globals, &queue_handle, shared_state, event_loop.handle())?;

        info_tx
          .send(CreateInfo {
            id: wayland_state.id(),
            loop_signal: event_loop.get_signal(),
          })
          .map_to_os_err()?;

        loop {
          if let Err(error) = event_loop.dispatch(None, &mut wayland_state) {
            log::error!("{error}");
          }

          if wayland_state.state_lock().should_exit {
            break Ok(());
          }
        }
      })
      .map_to_os_err()?;

    let CreateInfo { id, loop_signal } = info_rx.recv().map_to_os_err()?;

    Ok(Self { id, state, loop_signal })
  }
}

impl BackendWindow for WaylandWindow {
  fn id(&self) -> WindowId {
    self.id
  }

  fn raw_window_handle(&self) -> ventana_hal::raw_window_handle::RawWindowHandle {
    todo!()
  }

  fn raw_display_handle(&self) -> ventana_hal::raw_window_handle::RawDisplayHandle {
    todo!()
  }

  fn monitor(&self) -> Arc<dyn ventana_hal::monitor::BackendMonitor> {
    todo!()
  }

  fn next_event(&self) -> Option<ventana_hal::event::Event> {
    todo!()
  }

  fn iter<'w>(&'w self) -> Box<dyn ventana_hal::window::BackendEventIterator<'w> + 'w> {
    todo!()
  }

  fn close(&self) {
    todo!()
  }

  fn is_closing(&self) -> bool {
    todo!()
  }

  fn title(&self) -> String {
    todo!()
  }

  fn scale_factor(&self) -> f64 {
    todo!()
  }

  fn inner_size(&self) -> ventana_hal::dpi::Size {
    todo!()
  }

  fn outer_size(&self) -> ventana_hal::dpi::Size {
    todo!()
  }

  fn inner_position(&self) -> ventana_hal::dpi::Position {
    todo!()
  }

  fn outer_position(&self) -> ventana_hal::dpi::Position {
    todo!()
  }

  fn key(&self, keycode: ventana_hal::keyboard::Code) -> ventana_hal::keyboard::KeyState {
    todo!()
  }

  fn mouse(&self, button: ventana_hal::input::mouse::MouseButton) -> ventana_hal::keyboard::KeyState {
    todo!()
  }

  fn shift_key(&self) -> ventana_hal::keyboard::KeyState {
    todo!()
  }

  fn ctrl_key(&self) -> ventana_hal::keyboard::KeyState {
    todo!()
  }

  fn alt_key(&self) -> ventana_hal::keyboard::KeyState {
    todo!()
  }

  fn super_key(&self) -> ventana_hal::keyboard::KeyState {
    todo!()
  }
}
