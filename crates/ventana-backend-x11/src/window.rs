use {
  crate::{
    X11,
    event::map_native_event,
  },
  std::{
    num::NonZero,
    ptr::NonNull,
    sync::{
      Arc,
      Mutex,
      MutexGuard,
    },
  },
  ventana_hal::{
    backend::Backend,
    dpi::{
      PhysicalPosition,
      PhysicalSize,
    },
    error::{
      MapToOSError,
      RequestError,
    },
    event::{
      Event,
      WindowEvent,
    },
    monitor::BackendMonitor,
    raw_window_handle::*,
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
  is_running: bool,
}

pub struct X11Window {
  id: u32,
  visual: u32,
  state: Mutex<State>,
}

impl Drop for X11Window {
  fn drop(&mut self) {
    let _ = X11::connection().destroy_window(self.id);
  }
}

impl X11Window {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    let connection = X11::connection();
    let screen = X11::default_screen();
    let id = connection.generate_id().map_to_os_err()?;

    let clear_color: u32 = settings
      .clear_color
      .map(|color| (color.r as u32) << 16 | (color.g as u32) << 8 | (color.b as u32))
      .unwrap_or(screen.black_pixel);

    let visual = screen.root_visual;

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
    let scale_factor = X11::instance().primary_monitor().map_to_os_err()?.scale_factor();
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
      .change_property32(PropMode::REPLACE, id, X11::atoms().WM_PROTOCOLS, AtomEnum::ATOM, &[
        X11::atoms().WM_DELETE_WINDOW
      ])
      .map_to_os_err()?;

    connection.map_window(id).map_to_os_err()?;
    connection.flush().map_to_os_err()?;

    // loop {
    //   let event = connection.wait_for_event().map_to_os_err()?;
    //   log::trace!("{:?}", event);
    // }

    Ok(Self {
      id,
      visual,
      state: Mutex::new(State {
        settings,
        is_running: true,
      }),
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

  fn raw_window_handle(&self) -> RawWindowHandle {
    let mut window_handle = XcbWindowHandle::new(unsafe { NonZero::new_unchecked(self.id) });
    window_handle.visual_id = Some(unsafe { NonZero::new_unchecked(self.visual) });
    window_handle.into()
  }

  fn raw_display_handle(&self) -> RawDisplayHandle {
    XcbDisplayHandle::new(
      Some(unsafe { NonNull::new_unchecked(X11::connection().get_raw_xcb_connection()) }),
      X11::default_screen_id() as _,
    )
    .into()
  }

  fn monitor(&self) -> Arc<dyn BackendMonitor> {
    todo!()
  }

  fn next(&self) -> Option<Event> {
    if self.is_closing() {
      return None;
    }

    let x11_event = X11::connection()
      .wait_for_event()
      .inspect_err(|e| log::error!("{e}"))
      .ok()?;
    let event = map_native_event(&x11_event, self.id);

    if let Event::Window(WindowEvent::CloseRequest) = event
      && self.state_lock().settings.close_on_x
    {
      self.close();
    }

    Some(event)
  }

  fn request_redraw(&self) {
    todo!()
  }

  fn close(&self) {
    self.state_lock().is_running = false;
  }

  fn is_closing(&self) -> bool {
    !self.state_lock().is_running
  }

  fn title(&self) -> String {
    todo!()
  }

  fn scale_factor(&self) -> f64 {
    todo!()
  }

  fn inner_size(&self) -> PhysicalSize<u32> {
    X11::geometry()
      .map(|geometry| PhysicalSize::new(geometry.width as u32, geometry.height as u32))
      .unwrap_or_default()
  }

  fn outer_size(&self) -> PhysicalSize<u32> {
    todo!()
  }

  fn inner_position(&self) -> PhysicalPosition<i32> {
    todo!()
  }

  fn outer_position(&self) -> PhysicalPosition<i32> {
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
