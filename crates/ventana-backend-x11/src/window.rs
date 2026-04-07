mod iter;

use {
  self::iter::X11EventIterator,
  crate::{
    X11,
    event::map_native_event,
  },
  std::sync::{
    Arc,
    Mutex,
    MutexGuard,
  },
  ventana_hal::{
    backend::Backend,
    error::{
      MapToOSError,
      RequestError,
    },
    event::{
      Event,
      WindowEvent,
    },
    settings::WindowSettings,
    window::{
      BackendWindow,
      WindowId,
    },
  },
  x11rb::{
    COPY_DEPTH_FROM_PARENT,
    connection::Connection,
    protocol::xproto::{
      AtomEnum,
      ConnectionExt as _,
      CreateWindowAux,
      EventMask,
      Gravity,
      PropMode,
      WindowClass,
    },
    wrapper::ConnectionExt as _,
  },
};

pub struct State {
  settings: WindowSettings,
  running: bool,
}

pub struct X11Window {
  id: u32,
  state: Arc<Mutex<State>>,
}

impl X11Window {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    let x11 = X11::instance();
    let connection = x11.connection();
    let screen = x11.default_screen();
    let id = connection.generate_id().map_to_os_err()?;

    let clear_color: u32 = settings
      .clear_color
      .map(|color| (color.r as u32) << 16 | (color.g as u32) << 8 | (color.b as u32))
      .unwrap_or(screen.black_pixel);

    let values = CreateWindowAux::default()
      .event_mask(
        EventMask::EXPOSURE
          | EventMask::BUTTON_PRESS
          | EventMask::BUTTON_RELEASE
          | EventMask::POINTER_MOTION
          | EventMask::ENTER_WINDOW
          | EventMask::LEAVE_WINDOW
          | EventMask::KEY_PRESS
          | EventMask::KEY_RELEASE,
      )
      .win_gravity(Gravity::NORTH_WEST)
      .background_pixel(clear_color);
    let scale_factor = x11.primary_monitor().map_to_os_err()?.scale_factor();
    let size = settings.size.to_logical(scale_factor);
    let position = settings
      .position
      .map(|p| p.to_logical(scale_factor))
      .unwrap_or_default();

    connection
      .create_window(
        COPY_DEPTH_FROM_PARENT,
        id,
        screen.root,
        position.x,
        position.y,
        size.width,
        size.height,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &values,
      )
      .map_to_os_err()?;

    connection
      .change_property8(PropMode::REPLACE, id, AtomEnum::WM_NAME, AtomEnum::STRING, settings.title.as_bytes())
      .map_to_os_err()?;

    connection
      .change_property8(PropMode::REPLACE, id, AtomEnum::WM_ICON_NAME, AtomEnum::STRING, settings.title.as_bytes())
      .map_to_os_err()?;

    connection
      .change_property32(PropMode::REPLACE, id, x11.atoms().WM_PROTOCOLS, AtomEnum::ATOM, &[x11
        .atoms()
        .WM_DELETE_WINDOW])
      .map_to_os_err()?;

    connection.map_window(id).map_to_os_err()?;
    connection.flush().map_to_os_err()?;

    // loop {
    //   let event = connection.wait_for_event().map_to_os_err()?;
    //   log::trace!("{:?}", event);
    // }

    Ok(Self {
      id,
      state: Arc::new(Mutex::new(State {
        settings,
        running: true,
      })),
    })
  }

  fn state_lock(&self) -> MutexGuard<'_, State> {
    self.state.lock().unwrap()
  }
}

impl BackendWindow for X11Window {
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
    if self.is_closing() {
      return None;
    }

    let x11 = X11::instance();
    let event = match x11.connection().wait_for_event() {
      Ok(event) => map_native_event(&event, self.id),
      Err(e) => {
        log::error!("{e}");
        return None;
      },
    };

    if let Event::Window(WindowEvent::CloseRequest) = event
      && self.state_lock().settings.close_on_x
    {
      self.close();
    }

    Some(event)
  }

  fn iter<'w>(&'w self) -> Box<dyn ventana_hal::window::BackendEventIterator<'w> + 'w> {
    Box::new(X11EventIterator::new(self))
  }

  fn close(&self) {
    self.state_lock().running = false;
  }

  fn is_closing(&self) -> bool {
    !self.state_lock().running
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

  fn key(&self, _keycode: ventana_hal::keyboard::Code) -> ventana_hal::keyboard::KeyState {
    todo!()
  }

  fn mouse(&self, _button: ventana_hal::input::mouse::MouseButton) -> ventana_hal::keyboard::KeyState {
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
