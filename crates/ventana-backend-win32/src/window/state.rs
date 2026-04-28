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
  state: Mutex<State>,
}

impl SharedInternal {
  pub fn new(settings: WindowSettings) -> Arc<Self> {
    Arc::new(Self { state: Mutex::new(State::new(settings.clone())) })
  }

  pub fn state_lock(&self) -> MutexGuard<'_, State> {
    self.state.lock().unwrap()
  }
}

#[derive(Debug)]
pub struct State {
  pub(crate) close_on_x: bool,
}

impl State {
  fn new(settings: WindowSettings) -> Self {
    Self { close_on_x: settings.close_on_x }
  }
}
