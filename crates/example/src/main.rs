use ventana::{
  event::WindowEvent,
  prelude::*,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  initialize_logger();

  let window = Window::new(WindowOptions::default())?;

  while let Some(event) = window.next_event() {
    match event {
      Event::LoopExiting => {},
      Event::Window(WindowEvent::Keyboard { code, .. }) => {
        println!("{code:?}");
      },
      _ => (),
    }
  }

  Ok(())
}

fn initialize_logger() {
  env_logger::builder()
    .filter(None, log::LevelFilter::Trace)
    .format_source_path(true)
    .init();
}
