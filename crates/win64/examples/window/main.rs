use win64::{
  class::WindowClass,
  descriptor::WindowDescriptor,
  dpi::Size,
  message::pump::{MessagePump, PollingMode},
  procedure::WindowProcedure,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let class = WindowClass::default();

  class.spawn(
    WindowDescriptor::default()
      .with_title("Test")
      .with_size(Size::Logical((800.0, 500.0).into())),
    App::new(),
  )?;

  MessagePump::default().with_mode(PollingMode::Poll).run();

  Ok(())
}

struct App {}

impl App {
  fn new() -> Self {
    Self {}
  }
}

impl WindowProcedure for App {
  // fn on_message(&mut self, window: WindowHandle, message: &Message) -> Option<Response> {
  //   None
  // }
}
