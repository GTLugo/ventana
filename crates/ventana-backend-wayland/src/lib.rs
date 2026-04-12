#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))] // TODO: Swap this out for a stub impl on other platforms.

// mod event;
// pub mod window;

// use {
//   self::window::WaylandWindow,
//   std::{
//     collections::VecDeque,
//     sync::{
//       Arc,
//       LazyLock,
//     },
//   },
//   ventana_hal::{
//     backend::Backend,
//     error::RequestError,
//     monitor::BackendMonitor,
//     settings::WindowSettings,
//     window::BackendWindow,
//   },
// };

// #[derive(Clone)]
// pub struct Wayland;

// impl Backend for Wayland {
//   fn instance() -> &'static Self
//   where
//     Self: Sized,
//   {
//     static INSTANCE: LazyLock<Wayland> = LazyLock::new(|| Wayland);
//     &INSTANCE
//   }

//   fn name(&self) -> &'static str {
//     "Wayland"
//   }

//   fn create_window(&self, settings: WindowSettings) -> Result<Arc<dyn BackendWindow>, RequestError> {
//     Ok(Arc::new(WaylandWindow::new(settings)?))
//   }

//   fn list_available_monitors(&self) -> VecDeque<Arc<dyn BackendMonitor>> {
//     todo!()
//   }

//   fn primary_monitor(&self) -> Result<Arc<dyn BackendMonitor>, RequestError> {
//     todo!()
//   }
// }
