#![cfg(linux_platform)]

use {
  crate::{
    X11,
    keyboard::key_from_x11,
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
    keyboard::KeyState,
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
    protocol::{
      Event as X11Event,
      xproto::{
        AtomEnum,
        ConnectionExt,
        CreateWindowAux,
        EventMask,
        Gravity,
        PropMode,
        WindowClass,
      },
    },
    wrapper::ConnectionExt as _,
  },
};

pub struct State {
  settings: WindowSettings,
  size: PhysicalSize<u32>,
  position: PhysicalPosition<i32>,
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
    log::trace!("Destroyed window");
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
          | EventMask::KEYMAP_STATE
          | EventMask::STRUCTURE_NOTIFY
          | EventMask::FOCUS_CHANGE
          | EventMask::PROPERTY_CHANGE
          | EventMask::VISIBILITY_CHANGE
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
    let scale_factor = X11::instance().unwrap().primary_monitor().map_to_os_err()?.scale_factor();
    let size = settings.size.to_physical(scale_factor);
    let position = settings.position.map(|p| p.to_physical(scale_factor)).unwrap_or_default();

    connection
      .create_window(
        COPY_DEPTH_FROM_PARENT,
        id,
        screen.root,
        position.x as i16,
        position.y as i16,
        size.width as u16,
        size.height as u16,
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
      .change_property8(
        PropMode::REPLACE,
        id,
        AtomEnum::WM_ICON_NAME,
        AtomEnum::STRING,
        settings.title.as_bytes(),
      )
      .map_to_os_err()?;

    connection
      .change_property32(PropMode::REPLACE, id, X11::atoms().WM_PROTOCOLS, AtomEnum::ATOM, &[
        X11::atoms().WM_DELETE_WINDOW
      ])
      .map_to_os_err()?;

    connection.map_window(id).map_to_os_err()?;
    connection.flush().map_to_os_err()?;

    // let xkb = Context::new().ok_or(RequestError::not_supported("XKB not found"))?;

    let this = Self {
      id,
      visual,
      state: Mutex::new(State { settings, size, position, is_running: true }),
      // xkb, // request_redraw: AtomicBool::new(false),
    };

    this.request_redraw();

    Ok(this)
  }

  fn state_lock(&self) -> MutexGuard<'_, State> {
    self.state.lock().unwrap()
  }

  fn map_native_event(&self, native: &Option<X11Event>) -> Event {
    let Some(native) = native else {
      return Event::None;
    };
    match native {
      X11Event::ClientMessage(event) => {
        let data = event.data.as_data32();

        if event.window != self.id || event.format != 32 {
          return Event::None;
        }

        if data[0] == X11::atoms().WM_DELETE_WINDOW {
          // log::debug!("ClientMessage | Delete Window");
          return Event::Window(WindowEvent::CloseRequest);
        }

        if event.type_ == X11::atoms().VENTANA_REQUEST_REDRAW {
          // log::debug!("ClientMessage | Redraw");
          return Event::Window(WindowEvent::Draw);
        }

        Event::None
      },
      X11Event::Expose(event) => {
        if event.window == self.id {
          Event::Window(WindowEvent::Draw)
        } else {
          Event::None
        }
      },
      X11Event::ConfigureNotify(event) => {
        let mut state = self.state_lock();
        let old_size = state.size;
        let new_size = PhysicalSize::new(event.width as u32, event.height as u32);

        if event.window == self.id && old_size != new_size {
          state.size = new_size;
          return Event::Window(WindowEvent::Resized(new_size));
        }

        let old_position = state.position;
        let new_position = PhysicalPosition::new(event.x as i32, event.y as i32);

        if event.window == self.id && old_position != new_position {
          state.position = new_position;
          return Event::Window(WindowEvent::Moved(new_position));
        }

        Event::None
      },
      X11Event::ButtonPress(event) => {
        log::debug!("X11Event::ButtonPress | {event:?}");
        Event::None
      },
      X11Event::ButtonRelease(event) => {
        log::debug!("X11Event::ButtonRelease | {event:?}");
        Event::None
      },
      X11Event::KeyPress(event) => {
        // log::debug!("X11Event::KeyPress | {event:?}");
        match key_from_x11(event.detail, KeyState::Down) {
          Some(event) => Event::Window(WindowEvent::Keyboard(event)),
          None => Event::None,
        }
      },
      X11Event::KeyRelease(event) => {
        // log::debug!("X11Event::KeyRelease | {event:?}");
        match key_from_x11(event.detail, KeyState::Down) {
          Some(event) => Event::Window(WindowEvent::Keyboard(event)),
          None => Event::None,
        }
      },
      X11Event::Error(error) => {
        log::error!("{error:?}");
        Event::None
      },
      _ => {
        // if self.request_redraw.swap(false, Ordering::AcqRel) {
        //   log::debug!("Redraw requested");
        //   Event::Window(WindowEvent::Draw)
        // } else {
        Event::None
        // }
      },
    }
  }

  fn next_event<const SHOULD_WAIT: bool>(&self) -> Option<Event> {
    if self.is_closing() {
      return None;
    }

    let event = match SHOULD_WAIT {
      true => Some(X11::connection().wait_for_event().inspect_err(|e| log::error!("{e}")).ok()?),
      false => X11::connection().poll_for_event().inspect_err(|e| log::error!("{e}")).ok()?,
    };

    let event = self.map_native_event(&event);

    if let Event::Window(WindowEvent::CloseRequest) = event
      && self.state_lock().settings.close_on_x
    {
      self.close();
    }

    Some(event)
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
    self.next_event::<true>()
  }

  fn try_next(&self) -> Option<Event> {
    self.next_event::<false>()
  }

  fn request_redraw(&self) {
    X11::connection()
      .send_event(false, self.id, EventMask::EXPOSURE, x11rb::protocol::xproto::ClientMessageEvent {
        response_type: x11rb::protocol::xproto::CLIENT_MESSAGE_EVENT,
        format: 32,
        sequence: 0,
        window: self.id,
        type_: X11::atoms().VENTANA_REQUEST_REDRAW,
        data: [0; 5].into(),
      })
      .unwrap();
    // self.request_redraw.store(true, Ordering::Release);
  }

  fn close(&self) {
    self.state_lock().is_running = false;
  }

  fn is_closing(&self) -> bool {
    !self.state_lock().is_running
  }

  fn title(&self) -> String {
    let reply = X11::connection()
      .get_property(false, self.id, X11::atoms()._NET_WM_NAME, X11::atoms().UTF8_STRING, 0, u32::MAX)
      .unwrap()
      .reply()
      .unwrap();

    if !reply.value.is_empty() {
      return String::from_utf8_lossy(&reply.value).to_string();
    }

    let reply = X11::connection()
      .get_property(false, self.id, AtomEnum::WM_NAME, AtomEnum::STRING, 0, u32::MAX)
      .unwrap()
      .reply()
      .unwrap();

    if !reply.value.is_empty() {
      return String::from_utf8_lossy(&reply.value).to_string();
    }

    String::new()
  }

  fn set_title(&self, title: String) {
    let _ = title;
    todo!()
  }

  fn scale_factor(&self) -> f64 {
    todo!()
  }

  fn inner_size(&self) -> PhysicalSize<u32> {
    let geometry = X11::connection().get_geometry(self.id).unwrap().reply().unwrap();
    PhysicalSize::new(geometry.width as u32, geometry.height as u32)
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

  fn pointer(
    &self,
    _button: ventana_hal::pointer::button::PointerButton,
  ) -> ventana_hal::pointer::ButtonState {
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

  fn meta_key(&self) -> ventana_hal::keyboard::KeyState {
    todo!()
  }
}
