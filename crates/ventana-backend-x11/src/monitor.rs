use {
  crate::X11,
  std::{
    collections::VecDeque,
    ops::Deref,
    sync::Arc,
  },
  ventana_hal::{
    backend::Backend,
    monitor::{
      BackendMonitor,
      MonitorId,
    },
  },
  x11rb::protocol::randr::ConnectionExt,
};

#[derive(Debug, Clone)]
pub struct X11Monitor {
  id: u32,
  scale_factor: f64,
  primary: bool,
  automatic: bool,
  x: i16,
  y: i16,
  width: u16,
  height: u16,
  width_in_millimeters: u32,
  height_in_millimeters: u32,
}

impl X11Monitor {
  pub const DEFAULT_DPI: f64 = 96.0;

  fn list_available() -> VecDeque<Arc<Self>>
  where
    Self: Sized,
  {
    let x11 = X11::instance();
    let screen = x11.default_screen();
    let scale_factor = x11.get_xft_dpi() / Self::DEFAULT_DPI;
    x11
      .connection()
      .randr_get_monitors(screen.root, true)
      .unwrap()
      .reply()
      .unwrap()
      .monitors
      .into_iter()
      .map(|info| {
        Arc::new(Self {
          id: info.name,
          scale_factor,
          primary: info.primary,
          automatic: info.automatic,
          x: info.x,
          y: info.y,
          width: info.width,
          height: info.height,
          width_in_millimeters: info.width_in_millimeters,
          height_in_millimeters: info.height_in_millimeters,
        }) as _
      })
      .collect()
  }
}

impl BackendMonitor for X11Monitor {
  fn id(&self) -> MonitorId {
    MonitorId::from_raw(self.id as usize)
  }

  fn scale_factor(&self) -> f64 {
    self.scale_factor
  }

  fn list_available() -> VecDeque<Arc<dyn BackendMonitor>>
  where
    Self: Sized,
  {
    let x11 = X11::instance();
    let screen = x11.default_screen();
    let scale_factor = x11.get_xft_dpi() / Self::DEFAULT_DPI;
    x11
      .connection()
      .randr_get_monitors(screen.root, true)
      .unwrap()
      .reply()
      .unwrap()
      .monitors
      .into_iter()
      .map(|info| {
        Arc::new(Self {
          id: info.name,
          scale_factor,
          primary: info.primary,
          automatic: info.automatic,
          x: info.x,
          y: info.y,
          width: info.width,
          height: info.height,
          width_in_millimeters: info.width_in_millimeters,
          height_in_millimeters: info.height_in_millimeters,
        }) as _
      })
      .collect()
  }

  fn primary() -> impl BackendMonitor
  where
    Self: Sized,
  {
    let monitors = Self::list_available();
    log::debug!("Available monitors: {monitors:?}");
    let primary = monitors.iter().find(|m| m.primary);
    log::debug!("Primary: {primary:?}");
    primary
      .or_else(|| monitors.front())
      .expect("No available monitors to select from")
      .deref()
      .clone()
  }
}
