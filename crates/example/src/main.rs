use ventana::{event::WindowEvent, prelude::*};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let window = Window::builder()
    .with_settings(WindowSettings::default())
    .build()?;

  while let Some(event) = window.next_event() {
    match event {
      Event::LoopExiting => {}
      Event::Window(WindowEvent::Keyboard { code, .. })=> {
        println!("{code:?}");
      }
      _ => (),
    }
  }
  
  Ok(())
}
