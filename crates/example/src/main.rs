use ventana::prelude::*;

fn main() -> anyhow::Result<()> {
  example::initialize_logger();

  let window = Window::new(WindowOptions::default())?;

  for event in &window {
    log::info!("{} | {event:?}", window.id());
  }

  Ok(())
}
