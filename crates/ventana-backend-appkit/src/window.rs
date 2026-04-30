#![cfg(target_os = "macos")]

use {
  cacao::appkit::{
    App,
    AppDelegate,
    menu::{
      Menu,
      MenuItem,
    },
    window::Window,
  },
  std::sync::Arc,
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
    pointer::{
      ButtonState,
      mouse::MouseButton,
    },
    settings::WindowSettings,
    window::{
      BackendWindow,
      WindowId,
    },
  },
};

pub struct AppKitWindow {
  window: Window,
}

impl AppKitWindow {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    let _ = settings;

    App::new("com.gtlugo.window", Self { window: Default::default() }).run();

    Ok(Self { window: Default::default() })
  }
}

impl AppDelegate for AppKitWindow {
  fn did_finish_launching(&self) {
    App::set_menu(vec![
      Menu::new("", vec![
        MenuItem::Services,
        MenuItem::Separator,
        MenuItem::Hide,
        MenuItem::HideOthers,
        MenuItem::ShowAll,
        MenuItem::Separator,
        MenuItem::Quit,
      ]),
      Menu::new("File", vec![MenuItem::CloseWindow]),
      Menu::new("View", vec![MenuItem::EnterFullScreen]),
      Menu::new("Window", vec![
        MenuItem::Minimize,
        MenuItem::Zoom,
        MenuItem::Separator,
        MenuItem::new("Bring All to Front"),
      ]),
    ]);

    App::activate();

    self.window.set_minimum_content_size(400., 400.);
    self.window.set_title("A Basic Window");
    self.window.show();
  }

  fn should_terminate_after_last_window_closed(&self) -> bool {
    true
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

  // fn title(&self) -> String {
  //   todo!()
  // }

  fn set_title(&self, title: String) {
    self.window.set_title(&title);
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
