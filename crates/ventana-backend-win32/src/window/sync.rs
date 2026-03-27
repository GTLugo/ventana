use std::sync::{
  Arc,
  Condvar,
  Mutex,
};

#[derive(Clone)]
pub struct SyncData {
  new_event: Arc<(Mutex<bool>, Condvar)>,
  next_frame: Arc<(Mutex<bool>, Condvar)>,
  skip_wait: Arc<Mutex<bool>>,
}

impl SyncData {
  pub fn new() -> Self {
    Self {
      new_event: Arc::new((Mutex::new(false), Condvar::new())),
      next_frame: Arc::new((Mutex::new(true), Condvar::new())),
      skip_wait: Arc::new(Mutex::new(true)),
    }
  }

  pub fn signal_new_event(&self) {
    let (lock, cvar) = self.new_event.as_ref();
    let mut new = lock.lock().unwrap();
    if !*new {
      *new = true;
      cvar.notify_all();
    }
  }

  pub fn wait_on_new_event(&self) {
    let (lock, cvar) = self.new_event.as_ref();
    let mut new = cvar.wait_while(lock.lock().unwrap(), |new| !*new).unwrap();
    *new = false;
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

  pub fn skip_wait(&self, should_skip: bool) {
    *self.skip_wait.lock().unwrap() = should_skip;
  }
}
