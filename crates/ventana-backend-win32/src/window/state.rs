use {
  std::sync::{
    Arc,
    Mutex,
    MutexGuard,
    atomic::{
      AtomicBool,
      Ordering,
    },
  },
  ventana_hal::settings::WindowSettings,
};

#[derive(Debug)]
pub(crate) struct SharedInternal {
  is_ready: AtomicBool,
  state: Mutex<State>,
}

impl SharedInternal {
  pub fn new(settings: WindowSettings) -> Arc<Self> {
    Arc::new(Self {
      is_ready: AtomicBool::new(false),
      state: Mutex::new(State::new(settings.clone())),
    })
  }

  pub fn set_ready(&self, is_ready: bool) {
    self.is_ready.store(is_ready, Ordering::Release)
  }

  pub fn is_ready(&self) -> bool {
    self.is_ready.load(Ordering::Acquire)
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
    Self {
      close_on_x: settings.close_on_x,
    }
  }
}
