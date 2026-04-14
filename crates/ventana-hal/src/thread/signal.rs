use std::sync::{
  Arc,
  atomic::{
    AtomicBool,
    Ordering,
  },
};

#[derive(Default, Debug, Clone)]
pub struct StopSignal(Arc<AtomicBool>);

impl StopSignal {
  pub fn new() -> Self {
    Self(Arc::new(AtomicBool::new(false)))
  }

  pub fn should_stop(&self) -> bool {
    self.0.load(Ordering::Acquire)
  }

  pub fn stop(&self) {
    self.0.store(true, Ordering::Release);
  }
}
