use {
  ventana_hal::monitor::{
    BackendMonitor,
    MonitorId,
  },
  win64::{
    Handle,
    user::Monitor,
  },
};

pub struct Win32Monitor(pub(crate) Monitor);

impl BackendMonitor for Win32Monitor {
  fn id(&self) -> MonitorId {
    MonitorId::from_raw(self.0.to_raw())
  }

  fn scale_factor(&self) -> f64 {
    self.0.scale_factor()
  }
}
