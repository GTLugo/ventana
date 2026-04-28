#![cfg(linux_platform)]

use {
  std::sync::Arc,
  ventana_hal::{
    dpi::{
      PhysicalPosition,
      PhysicalSize,
    },
    event::Event,
    keyboard::{
      Code,
      KeyState,
    },
    pointer::{
      ButtonState,
      mouse::MouseButton,
    },
    window::{
      BackendWindow,
      WindowId,
    },
  },
};

pub struct WaylandWindow;

impl BackendWindow for WaylandWindow {
  fn id(&self) -> WindowId {
    todo!()
  }

  fn raw_window_handle(&self) -> ventana_hal::raw_window_handle::RawWindowHandle {
    todo!()
  }

  fn raw_display_handle(&self) -> ventana_hal::raw_window_handle::RawDisplayHandle {
    todo!()
  }

  fn monitor(&self) -> Arc<dyn ventana_hal::monitor::BackendMonitor> {
    todo!()
  }

  fn next(&self) -> Option<Event> {
    todo!()
  }

  fn request_redraw(&self) {
    todo!()
  }

  fn close(&self) {
    todo!()
  }

  fn is_closing(&self) -> bool {
    todo!()
  }

  // fn title(&self) -> String {
  //   todo!()
  // }

  fn set_title(&self, title: String) {
    let _ = title;
    todo!()
  }

  fn scale_factor(&self) -> f64 {
    todo!()
  }

  fn inner_size(&self) -> PhysicalSize<u32> {
    todo!()
  }

  fn outer_size(&self) -> PhysicalSize<u32> {
    todo!()
  }

  fn inner_position(&self) -> PhysicalPosition<i32> {
    todo!()
  }

  fn outer_position(&self) -> PhysicalPosition<i32> {
    todo!()
  }

  fn key(&self, keycode: Code) -> KeyState {
    let _ = keycode;
    todo!()
  }

  fn mouse(&self, button: MouseButton) -> ButtonState {
    let _ = button;
    todo!()
  }

  fn shift_key(&self) -> KeyState {
    todo!()
  }

  fn ctrl_key(&self) -> KeyState {
    todo!()
  }

  fn alt_key(&self) -> KeyState {
    todo!()
  }

  fn super_key(&self) -> KeyState {
    todo!()
  }
}
