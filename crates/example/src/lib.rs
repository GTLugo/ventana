use ventana::window::Window;

pub fn initialize_logger() {
  env_logger::builder()
    .filter(None, log::LevelFilter::Trace)
    .format_source_path(true)
    .init();
}

pub struct State {
  window: Window,
}

impl State {
  // We don't need this to be async right now,
  // but we will in the next tutorial
  pub async fn new(window: Window) -> anyhow::Result<Self> {
    Ok(Self { window })
  }

  pub fn resize(&mut self, width: u32, height: u32) {
    log::info!("Resize: ({width}, {height})");
    // We'll do stuff here in the next tutorial
  }

  pub fn render(&mut self) {
    self.window.request_redraw();

    // We'll do more stuff here in the next tutorial
  }
}
