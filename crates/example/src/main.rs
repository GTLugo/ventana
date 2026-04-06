use ventana::prelude::*;

fn main() -> anyhow::Result<()> {
  example::initialize_logger();

  let window = Window::new(WindowOptions {
    title: "Example",
    size: Size::Logical((800, 500).into()),
    clear_color: Some((50, 50, 50).into()),
    ..Default::default()
  })?;

  for event in &window {
    log::error!("NEW FRAME");
    log::info!("{window} | {event:?}");
  }

  Ok(())
}
