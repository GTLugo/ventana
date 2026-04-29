use std::sync::{
  Arc,
  Condvar,
  Mutex,
};

impl Default for AcknowledgeSignal {
  fn default() -> Self {
    Self::new()
  }
}

#[derive(Debug, Clone)]
pub struct AcknowledgeSignal(Arc<(Mutex<bool>, Condvar)>);

impl AcknowledgeSignal {
  pub fn new() -> Self {
    Self(Arc::new((Mutex::new(true), Condvar::new())))
  }

  pub fn heard(self) {
    let (lock, cvar) = &*self.0;
    let mut pending = lock.lock().unwrap();
    *pending = false;
    cvar.notify_one();
  }

  pub fn wait(self) {
    let (lock, cvar) = &*self.0;
    let pending = lock.lock().unwrap();
    let _guard = cvar.wait_while(pending, |pending| *pending).unwrap();
  }
}
