use {
  example::State,
  ventana::{
    backend::Backend,
    prelude::*,
  },
};

fn main() -> anyhow::Result<()> {
  example::initialize_logger();

  log::debug!("Backend: {}", Backend::auto()?.name());

  {
    let window = Window::new(WindowOptions {
      title: "Example",
      size: Size::Logical((800, 500).into()),
      ..Default::default()
    })?;

    // let mut state: Option<State> = None;

    let mut state = pollster::block_on(State::new(window.clone()))?;

    for event in &window {
      if let Event::Window(event) = event {
        log::info!("{window} | {event:?}");
        match event {
          // TODO: This probably shouldn't be an event and should instead be expected as completed
          //       by the time Window::new returns. That way graphics contexts may be created and
          //       window handles are valid. So, just remove this event so that WM_CREATE won't block.
          // WindowEvent::Created => {
          //   state.replace(pollster::block_on(State::new(window.clone()))?);
          // }
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
  }

  Ok(())
}
