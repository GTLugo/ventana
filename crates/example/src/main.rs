use ventana::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  example::initialize_logger();

  let window = Window::new(WindowOptions::default())?;

  for event in &window {
    match event {
      Event::LoopExiting => {},
      Event::Window(WindowEvent::Keyboard { code, .. }) => {
        println!("{code:?}");
      },
      _ => (),
    }
  }

  // while let Some(event) = window.next_event() {}

  Ok(())
}
