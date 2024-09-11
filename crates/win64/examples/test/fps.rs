use std::time::{Duration, Instant};

#[allow(unused)]
pub struct FPSCounter {
  then: Instant,
  delta: Duration,
  timer: Duration,
  period: Duration,
}

impl Default for FPSCounter {
  fn default() -> Self {
    Self::new()
  }
}

#[allow(unused)]
impl FPSCounter {
  pub fn new() -> Self {
    Self {
      then: Instant::now(),
      delta: Duration::ZERO,
      timer: Duration::ZERO,
      period: Duration::from_secs_f64(0.2),
    }
  }

  fn tick(&mut self) {
    let now = Instant::now();
    self.delta = now - self.then;
    self.then = now;
    self.timer += self.delta;
  }

  pub fn update(&mut self, mut f: impl FnMut(&mut FPSCounter)) {
    self.tick();
    f(self);
  }

  pub fn timer_up(&self) -> bool {
    self.timer > self.period
  }

  pub fn reset_timer(&mut self) {
    self.timer = Duration::ZERO;
  }

  pub fn delta(&self) -> Duration {
    self.delta
  }
}
