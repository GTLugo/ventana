use std::sync::{Arc, Mutex};

use ventana_hal::{event::Event, settings::WindowSettings};

use super::sync::SyncData;

pub(crate) struct CreateInfo {
  pub settings: WindowSettings,
  pub message: Arc<Mutex<Option<Event>>>,
  pub sync: SyncData,
  // pub style: Style,
}