use {
  example::{
    State,
    logger::Logger,
  },
  ventana::prelude::*,
};

fn main() -> anyhow::Result<()> {
  Logger::init()?;

  AutoBackend::instance().inspect(|b| log::debug!("Backend: {}", b.name()));

  let window = Window::new(
    WindowOptions::default()
      .with_title("Example")
      .with_size(LogicalSize::new(800, 500))
      .with_clear_color(WindowOptions::BLACK),
  )?;

  let mut state = pollster::block_on(State::new(window.clone()))?;

  for event in &window {
    if let Event::Window(event) = event {
      if !matches!(event, WindowEvent::Draw) {
        log::debug!("{window:?} | {event:?}");
      }

      match event {
        WindowEvent::Draw => {
          state.update();
          state.draw();
          window.request_redraw();
        },
        WindowEvent::Resized(physical_size) => {
          state.resize(physical_size);
        },
        _ => (),
      }
    }
  }

  Ok(())
}
