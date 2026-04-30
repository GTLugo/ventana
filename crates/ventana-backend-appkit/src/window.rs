#![cfg(target_os = "macos")]

use {
  dispatch2::MainThreadBound, objc2::{MainThreadMarker, rc::autoreleasepool}, objc2_app_kit::{
    NSScreen, NSWindow, NSWindowStyleMask
  }, objc2_core_foundation::{
    CGPoint,
    CGRect,
    CGSize,
  }, std::sync::Arc, ventana_hal::{
    dpi::{
      PhysicalPosition,
      PhysicalSize,
      Position,
    },
    error::RequestError,
    event::Event,
    keyboard::{
      Code,
      KeyState,
    },
    pointer::{
      ButtonState,
      mouse::MouseButton,
    },
    settings::WindowSettings,
    window::{
      BackendWindow,
      WindowId,
    },
  }
};

pub struct AppKitWindow {}

impl AppKitWindow {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    let _ = settings;

    let mtm =
      MainThreadMarker::new().ok_or(RequestError::NotSupported("Marker must be on main thread.".into()))?;

    let screen =
      NSScreen::mainScreen(mtm).ok_or(RequestError::NotSupported("No main screen found.".into()))?;

    let style = NSWindowStyleMask::Closable
      | NSWindowStyleMask::Titled
      | NSWindowStyleMask::Miniaturizable
      | NSWindowStyleMask::Resizable;

    // log::info!("Backing scale factor: {}", screen.backingScaleFactor());

    let scale_factor = screen.backingScaleFactor();

    let physical_size = settings.size.to_physical(scale_factor);
    let size = CGSize::new(physical_size.width, physical_size.height);

    let physical_pos = match settings.position {
      Some(pos) => {
        let mut pos = pos.to_physical(scale_factor);

        pos.x += (physical_size.width / 2.0) as i32;
        pos.y += (physical_size.height / 2.0) as i32;

        Position::Physical(pos)
      },
      None => Position::Physical(
        ((screen.frame().size.width - size.width) / 2.0, (screen.frame().size.height - size.height) / 2.0)
          .into(),
      ),
    }
    .to_physical(scale_factor);
    let origin = CGPoint::new(physical_pos.x, physical_pos.y);

    let content_rect = CGRect::new(origin, size).standardize();

    Ok(Self {})
  }
}

impl BackendWindow for AppKitWindow {
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

  fn title(&self) -> String {
    todo!()
  }

  fn set_title(&self, title: String) {
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
