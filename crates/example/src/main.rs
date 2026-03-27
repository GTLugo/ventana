use ventana::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  example::initialize_logger();

  let window = Window::new(WindowOptions::default())?;

  for event in &window {
    log::info!("{event:?}");
  }

  Ok(())
}
