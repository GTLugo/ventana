#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))]

use {
  crate::X11,
  std::collections::VecDeque,
  ventana_hal::monitor::{
    BackendMonitor,
    MonitorId,
  },
  x11rb::protocol::randr::ConnectionExt,
};

#[allow(unused)]
#[derive(Debug, Clone)]
pub struct X11Monitor {
  pub(crate) id: u32,
  pub(crate) scale_factor: f64,
  pub(crate) primary: bool,
  pub(crate) automatic: bool,
  pub(crate) x: i16,
  pub(crate) y: i16,
  pub(crate) width: u16,
  pub(crate) height: u16,
  pub(crate) width_in_millimeters: u32,
  pub(crate) height_in_millimeters: u32,
}

impl X11Monitor {
  pub const DEFAULT_DPI: f64 = 96.0;

  pub fn list_available() -> VecDeque<Self>
  where
    Self: Sized,
  {
    let scale_factor = Self::get_xft_dpi() / Self::DEFAULT_DPI;
    X11::connection()
      .randr_get_monitors(X11::default_screen().root, true)
      .unwrap()
      .reply()
      .unwrap()
      .monitors
      .into_iter()
      .map(|info| Self {
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
      })
      .collect()
  }

  pub fn get_xft_dpi() -> f64 {
    X11::database()
      .get_value::<f64>("Xft.dpi", "")
      .ok()
      .flatten()
      .or_else(|| X11::database().get_value::<f64>("Xft/DPI", "").ok().flatten())
      .map(|dpi| if dpi > 0.0 { dpi } else { X11Monitor::DEFAULT_DPI })
      .unwrap_or(X11Monitor::DEFAULT_DPI)
  }
}

impl BackendMonitor for X11Monitor {
  fn id(&self) -> MonitorId {
    MonitorId::from_raw(self.id as usize)
  }

  fn scale_factor(&self) -> f64 {
    self.scale_factor
  }
}
