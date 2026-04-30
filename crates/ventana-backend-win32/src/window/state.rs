use {
  std::sync::{
    Arc,
    Mutex,
    MutexGuard,
  },
  ventana_hal::settings::WindowSettings,
};

#[derive(Debug)]
pub(crate) struct SharedInternal {
  pub title: Mutex<String>,
  pub close_on_x: bool,
  // state: Mutex<State>,
}

impl SharedInternal {
  pub fn new(settings: WindowSettings) -> Arc<Self> {
    let title = Mutex::new(settings.title.to_string());
    let close_on_x = settings.close_on_x;

    Arc::new(Self { title, close_on_x })
  }

  pub fn title(&self) -> MutexGuard<'_, String> {
    self.title.lock().unwrap()
  }
}

// #[derive(Debug)]
// pub struct State {
//   pub(crate) close_on_x: bool,
// }

// impl State {
//   fn new(settings: WindowSettings) -> Self {
//     Self { close_on_x: settings.close_on_x }
//   }
// }
