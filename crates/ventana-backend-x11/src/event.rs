use {
  ventana_hal::event::WindowEvent,
  x11rb::protocol::Event,
};

pub fn map_native_event(native: &Event) -> Option<WindowEvent> {
  let _ = native;
  todo!()
}
