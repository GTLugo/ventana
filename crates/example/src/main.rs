use {
  example::{
    State,
    logger::Logger,
  },
  ventana::prelude::*,
};

fn main() -> anyhow::Result<()> {
  Logger::init()?;

  // AutoBackend::set_preferences(&[Backend::Win32, Backend::Wayland, Backend::X11, Backend::AppKit]);
  AutoBackend::instance().inspect(|b| log::debug!("Backend: {}", b.name()));

  let window = Window::new(
    WindowOptions::default()
      .with_title("Example")
      .with_size(LogicalSize::new(800, 500))
      .with_clear_color(WindowOptions::BLACK),
  )?;

  let mut state = pollster::block_on(State::new(window.clone()))?;

  for event in window.try_iter() {
    if let Event::Window(event) = event {
      if !matches!(event, WindowEvent::Draw) {
        log::debug!("{:?} | {event:?}", window.title());
      }

      match event {
        WindowEvent::Draw => {
          // log::debug!("{:?} | {event:?}", window.title());
          // window.request_redraw();
        },
        WindowEvent::Resized(physical_size) => {
          state.resize(physical_size);
        },
        _ => (),
      }
    }
    state.update();
    state.draw();
  }

  Ok(())
}
