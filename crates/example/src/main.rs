use ventana::{
  backend::Backend,
  prelude::*,
};

fn main() -> anyhow::Result<()> {
  example::initialize_logger();

  log::debug!("Backend: {}", Backend::auto().unwrap().name());

  let window = Window::new(WindowOptions {
    title: "Example",
    size: Size::Logical((800, 500).into()),
    clear_color: Some((50, 50, 50).into()),
    ..Default::default()
  })?;

  for event in &window {
    if let Event::Window(event) = event {
      log::info!("{window} | {event:?}");
    }
  }

  Ok(())
}
