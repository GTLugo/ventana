#![cfg(linux_platform)]

pub mod state;

use {
  self::state::WindowState,
  crate::{
    Wayland,
    backend::WaylandConnection,
  },
  sctk::{
    compositor::CompositorState,
    reexports::protocols::xdg::activation::v1::client::xdg_activation_v1::XdgActivationV1,
    shell::xdg::window::Window as SctkWindow,
  },
  std::sync::{
    Arc,
    Mutex,
    atomic::AtomicBool,
  },
  ventana_hal::{
    dpi::{
      PhysicalPosition,
      PhysicalSize,
    },
    error::RequestError,
    event::Event,
    keyboard::{
      Code,
      KeyState,
    },
    monitor::Monitor,
    pointer::{
      ButtonState,
      button::PointerButton,
    },
    raw_window_handle::{
      WaylandDisplayHandle,
      WaylandWindowHandle,
    },
    settings::WindowSettings,
    window::{
      BackendWindow,
      WindowId,
    },
  },
  wayland_client::{
    Proxy,
    QueueHandle,
    protocol::wl_display::WlDisplay,
  },
};

pub struct WaylandWindow {
  window: SctkWindow,
  id: WindowId,
  state: Arc<Mutex<WindowState>>,
  compositor: Arc<CompositorState>,
  display: WlDisplay,
  xdg_activation: Option<XdgActivationV1>,
  attention_requested: Arc<AtomicBool>,
  queue_handle: QueueHandle<WaylandConnection>,
  monitors: Arc<Mutex<Vec<Monitor>>>,
}

impl WaylandWindow {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    let _ = settings;

    Ok(todo!())
  }
}

impl BackendWindow for WaylandWindow {
  fn id(&self) -> WindowId {
    todo!()
  }

  fn raw_window_handle(&self) -> ventana_hal::raw_window_handle::RawWindowHandle {
    WaylandWindowHandle::new(todo!()).into()
  }

  fn raw_display_handle(&self) -> ventana_hal::raw_window_handle::RawDisplayHandle {
    let display = Wayland::display().id().as_ptr();
    WaylandDisplayHandle::new(std::ptr::NonNull::new(display as *mut _).unwrap()).into()
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

  fn pointer(&self, button: PointerButton) -> ButtonState {
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

  fn meta_key(&self) -> KeyState {
    todo!()
  }
}
