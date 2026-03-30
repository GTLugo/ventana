mod state;

use {
  self::state::WaylandWindowState,
  sctk::reexports::client::{
    Connection,
    globals::{
      self,
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

pub struct WaylandWindow {
  id: u32,
  state: Arc<Mutex<WaylandWindowState>>,
}

impl WaylandWindow {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    // I can't find any decent beginner materials on how to actually make a Wayland window and read events,
    // so this is HEAVILY based on Winit. If there are any quirks, it's probably because of me
    // trying to warp Winit's implementation.

    let connection = Connection::connect_to_env().map_to_os_err()?;
    let (globals, mut event_queue) = globals::registry_queue_init(&connection).map_to_os_err()?;
    let queue_handle = event_queue.handle();

    let mut state = WaylandWindowState::new(&globals, &queue_handle)?;

    event_queue.roundtrip(&mut state).map_to_os_err()?;

    Ok(Self {
      id: todo!(),
      state: Arc::new(Mutex::new(state)),
    })
  }
}

impl BackendWindow for WaylandWindow {
  fn id(&self) -> WindowId {
    WindowId::from_raw(self.id as usize)
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
