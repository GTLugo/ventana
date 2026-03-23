use std::sync::{
  Arc,
  Condvar,
  Mutex,
};

#[derive(Clone)]
pub struct SyncData {
  pub new_message: Arc<(Mutex<bool>, Condvar)>,
  pub next_frame: Arc<(Mutex<bool>, Condvar)>,
  pub skip_wait: Arc<Mutex<bool>>,
}

impl SyncData {
  pub fn new() -> Self {
    Self {
      new_message: Arc::new((Mutex::new(false), Condvar::new())),
      next_frame: Arc::new((Mutex::new(true), Condvar::new())),
      skip_wait: Arc::new(Mutex::new(true)),
    }
  }

  pub fn signal_new_message(&self) {
    let (lock, cvar) = self.new_message.as_ref();
    let mut new = lock.lock().unwrap();
    if !*new {
      *new = true;
      cvar.notify_all();
    }
  }

  pub fn wait_on_frame(&self) {
    let (lock, cvar) = self.next_frame.as_ref();
    let mut next = cvar.wait_while(lock.lock().unwrap(), |next| !*next).unwrap();
    *next = *self.skip_wait.lock().unwrap();
  }

  pub fn signal_next_frame(&self) {
    let (lock, cvar) = self.next_frame.as_ref();
    let mut next = lock.lock().unwrap();
    if !*next {
      *next = true;
      cvar.notify_all();
    }
  }
}
