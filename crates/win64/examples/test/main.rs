use win64::prelude::*;

mod fps;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let class = WindowClass::new(&WindowClassDescriptor::default());

  class.spawn(WindowDescriptor::default().with_title("Test").with_size((800, 500)), WindowState::new())?;

  MessagePump::default().with_strategy(PumpStrategy::Poll).run();

  Ok(())
}

struct WindowState {}

impl WindowState {
  fn new() -> Self {
    Self {}
  }
}

impl WindowProcedure for WindowState {
  fn on_message(&mut self, window: WindowId, message: Message) -> ProcedureResult {
    match message {
      Message::CloseRequested => window.quit(),
      _ => {
        println!("{message:?}");
      }
    }

    window.default_procedure(message)
  }
}
