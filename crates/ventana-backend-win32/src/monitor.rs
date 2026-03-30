use {
  std::{
    collections::VecDeque,
    sync::Arc,
  },
  ventana_hal::monitor::{
    BackendMonitor,
    MonitorId,
  },
  win64::{
    Handle,
    user::Monitor,
  },
};

pub struct Win32Monitor {
  monitor: Monitor,
}

impl BackendMonitor for Win32Monitor {
  fn id(&self) -> MonitorId {
    MonitorId::from_raw(self.monitor.to_raw())
  }

  fn scale_factor(&self) -> f64 {
    self.monitor.scale_factor()
  }

  fn list_available() -> VecDeque<Arc<dyn BackendMonitor>>
  where
    Self: Sized,
  {
    Monitor::available()
      .iter()
      .map(|&monitor| Arc::new(Self { monitor }) as _)
      .collect()
  }

  fn primary() -> impl BackendMonitor
  where
    Self: Sized,
  {
    Self {
      monitor: Monitor::primary(),
    }
  }
}
