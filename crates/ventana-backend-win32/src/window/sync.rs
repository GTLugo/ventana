use synchronize::{
  ConditionalSignal,
  Signal,
};

#[derive(Clone)]
pub struct SyncData {
  pub new_event: Signal,
  pub next_frame: ConditionalSignal,
}

impl SyncData {
  pub fn new() -> Self {
    let next_frame = ConditionalSignal::new();
    next_frame.should_wait(false).unwrap();

    Self {
      new_event: Signal::new(),
      next_frame,
    }
  }
}
