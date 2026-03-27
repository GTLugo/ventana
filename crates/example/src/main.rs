use ventana::prelude::*;

fn main() -> anyhow::Result<()> {
  example::initialize_logger();

  let window = Window::new(WindowOptions {
    flow: Flow::Poll,
    ..Default::default()
  })?;

  for event in &window {
    if !matches!(event, Event::None) {
      log::info!("{window} | {event:?}");
    }
  }

  Ok(())
}
