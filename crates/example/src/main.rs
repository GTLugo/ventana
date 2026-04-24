use {
  example::State,
  ventana::prelude::*,
};

fn main() -> anyhow::Result<()> {
  example::initialize_logger();

  AutoBackend::instance().inspect(|b| log::debug!("Backend: {b:?}"));

  let window = Window::new(
    WindowOptions::default()
      // .with_backend(AutoBackend::instance())
      .with_title("Example")
      .with_size(LogicalSize::new(800, 500))
      .with_clear_color((0, 0, 0)),
  )?;

  let mut state = pollster::block_on(State::new(window.clone()))?;

  for event in &window {
    if let Event::Window(event) = event {
      log::info!("{event:?}");

      match event {
        WindowEvent::Draw => {
          state.update();
          state.draw();
        },
        WindowEvent::Resized(physical_size) => {
          state.resize(physical_size);
          state.draw();
        },
        _ => (),
      }
    }
  }

  Ok(())
}
