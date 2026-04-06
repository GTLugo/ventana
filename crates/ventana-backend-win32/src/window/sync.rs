use {
  std::sync::{
    Arc,
    Mutex,
  },
  synchronize::Signal,
};

#[derive(Clone)]
pub struct SyncData {
  pub new_event: Signal,
  pub next_frame: Signal,
  skip_wait: Arc<Mutex<bool>>,
}

impl SyncData {
  pub fn new() -> Self {
    Self {
      new_event: Signal::new(),
      next_frame: Signal::new(),
      skip_wait: Arc::new(Mutex::new(true)),
    }
  }

  pub fn skip_wait(&self, should_skip: bool) {
    *self.skip_wait.lock().unwrap() = should_skip;
  }
}
