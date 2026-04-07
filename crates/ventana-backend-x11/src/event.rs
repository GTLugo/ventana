use {
  ventana_hal::event::{
    Event,
    WindowEvent,
  },
  x11rb::protocol::Event as X11Event,
};

pub fn map_native_event(native: &X11Event) -> Event {
  match native {
    X11Event::CreateNotify(_) => Event::Window(WindowEvent::Created),
    X11Event::Error(error) => {
      log::error!("{error:?}");
      Event::None
    },
    _ => Event::None,
  }
}
