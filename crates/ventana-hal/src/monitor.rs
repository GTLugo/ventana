use std::sync::Arc;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MonitorId(usize);

impl MonitorId {
  pub const fn to_raw(self) -> usize {
    self.0
  }

  pub const fn from_raw(raw: usize) -> Self {
    Self(raw)
  }
}

impl std::fmt::Display for MonitorId {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}", self.0)
  }
}

pub trait BackendMonitor: Send + Sync {
  fn id(&self) -> MonitorId;

  fn scale_factor(&self) -> f64;
}

#[derive(Clone)]
pub struct Monitor
where
  Self: Send + Sync,
{
  monitor: Arc<dyn BackendMonitor>,
}

impl Monitor {
  pub fn new(monitor: Arc<dyn BackendMonitor>) -> Self {
    Self { monitor }
  }

  pub fn id(&self) -> MonitorId {
    self.monitor.id()
  }

  pub fn scale_factor(&self) -> f64 {
    self.monitor.scale_factor()
  }
}
