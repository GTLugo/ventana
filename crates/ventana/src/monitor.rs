use {
  std::sync::Arc,
  ventana_hal::monitor::{
    BackendMonitor,
    MonitorId,
  },
};

#[derive(Clone)]
pub struct Monitor
where
  Self: Send + Sync,
{
  monitor: Arc<dyn BackendMonitor>,
}

impl Monitor {
  pub(crate) fn new(monitor: Arc<dyn BackendMonitor>) -> Self {
    Self { monitor }
  }

  pub fn id(&self) -> MonitorId {
    self.monitor.id()
  }

  pub fn scale_factor(&self) -> f64 {
    self.monitor.scale_factor()
  }
}
