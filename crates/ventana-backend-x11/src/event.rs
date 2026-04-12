use {
  crate::X11,
  ventana_hal::event::{
    Event,
    WindowEvent,
  },
  x11rb::protocol::Event as X11Event,
};

pub fn map_native_event(native: &X11Event, window_id: u32) -> Event {
  match native {
    // X11Event::CreateNotify(_) => Event::Window(WindowEvent::Created),
    // X11Event::DestroyNotify(_) => Event::Window(WindowEvent::Destroyed),
    X11Event::ClientMessage(event) => {
      let data = event.data.as_data32();
      if event.format == 32 && event.window == window_id && data[0] == X11::atoms().WM_DELETE_WINDOW {
        Event::Window(WindowEvent::CloseRequest)
      } else {
        Event::None
      }
    },
    X11Event::Error(error) => {
      log::error!("{error:?}");
      Event::None
    },
    _ => Event::None,
  }
}
