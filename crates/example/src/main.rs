use {
  example::State,
  ventana::{
    backend::Backend,
    prelude::*,
  },
};

fn main() -> anyhow::Result<()> {
  example::initialize_logger();

  log::debug!("Backend: {}", Backend::auto().unwrap().name());

  let window = Window::new(WindowOptions {
    title: "Example",
    size: Size::Logical((800, 500).into()),
    clear_color: Some((80, 80, 80).into()),
    ..Default::default()
  })?;

  let mut state = pollster::block_on(State::new(window.clone()))?;

  for event in &window {
    if let Event::Window(event) = event {
      // log::info!("{window} | {event:?}");
      match event {
        WindowEvent::Draw => {
          state.update();
          state.draw();
        },
        WindowEvent::Resized(physical_size) => {
          state.resize(physical_size.width, physical_size.height);
          state.draw();
        },
        _ => (),
      }
    }
  }

  Ok(())
}
