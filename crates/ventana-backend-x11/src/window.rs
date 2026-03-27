use {
  std::sync::Arc,
  ventana_hal::{
    error::{
      MapToOSError,
      RequestError,
    },
    window::{
      BackendWindow,
      WindowId,
    },
  },
  x11rb::{
    COPY_DEPTH_FROM_PARENT,
    connection::Connection,
    protocol::xproto::{
      ConnectionExt,
      CreateWindowAux,
      WindowClass,
    },
  },
};

pub struct X11Window {
  id: u32,
}

impl X11Window {
  #[allow(clippy::new_ret_no_self)]
  pub fn new() -> Result<Arc<dyn BackendWindow>, RequestError> {
    let (connection, screen_index) = x11rb::connect(None).map_to_os_err()?;
    let screen = &connection.setup().roots[screen_index];
    let id = connection.generate_id().map_to_os_err()?;
    connection
      .create_window(
        COPY_DEPTH_FROM_PARENT,
        id,
        screen.root,
        0,
        0,
        800,
        500,
        0,
        WindowClass::INPUT_OUTPUT,
        0,
        &CreateWindowAux::new().background_pixel(screen.white_pixel),
      )
      .map_to_os_err()?;

    connection.map_window(id).map_to_os_err()?;
    connection.flush().map_to_os_err()?;

    loop {
      let event = connection.wait_for_event().map_to_os_err()?;
      log::trace!("{:?}", event);
    }

    Ok(Arc::new(Self { id }))
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
