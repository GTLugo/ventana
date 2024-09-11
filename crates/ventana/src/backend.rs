// #[cfg(all(
//   not(windows_platform),
//   not(x11_platform),
// ))]
// compile_error!("The platform you're compiling for is not supported by ventana. Make sure to double check your set feature flags as well.");

// #[cfg(windows_platform)]
// mod win32;

// #[cfg(x11_platform)]
// mod x11;

#[cfg(windows_platform)]
pub use ventana_backend_win32 as win32;

#[cfg(x11_platform)]
pub use ventana_backend_x11 as x11;

use ventana_hal::{Backend as HalBackend, Window as HalWindow};
use ventana_types::settings::WindowSettings;

// pub struct BackendRegistry {
//   backends: HashMap<TypeId, Box<dyn Backend>>,
// }

// impl Default for BackendRegistry {
//   fn default() -> Self {
//     let mut registry = Self::new();
//     #[cfg(windows_platform)]
//     {
//       // find a way to get the compiler check inside the function? maybe with a const generic?
//       registry = registry.register(win32::Win32Backend);
//     }
//     #[cfg(x11_platform)]
//     {
//       registry = registry.register(x11::X11Backend);
//     }
//     registry
//   }
// }

// impl BackendRegistry {
//   pub fn new() -> Self {
//     Self {
//       backends: HashMap::new(),
//     }
//   }

//   pub fn register<B: Backend + 'static>(mut self, backend: B) -> Self {
//     let id = TypeId::of::<B>();
//     self.backends.insert(id, Box::new(backend));
//     self
//   }

//   pub fn unregister<B: Backend + 'static>(mut self) -> Self {
//     let id = TypeId::of::<B>();
//     self.backends.remove(&id);
//     self
//   }

//   pub fn get<B: Backend + 'static>(&self) -> Option<&dyn Backend> {
//     let type_id = TypeId::of::<B>();
//     self.backends.get(&type_id).map(|b| b.as_ref())
//   }

//   pub fn get_all(&self) -> Vec<&dyn Backend> {
//     self.backends.values().map(|b| b.as_ref()).collect()
//   }
// }

pub struct DummyBackend;

impl HalBackend for DummyBackend {
  fn name() -> &'static str {
    "Dummy"
  }

  fn create_window(&self, settings: WindowSettings) -> Box<dyn HalWindow> {
    Box::new(DummyWindow::new(settings))
  }
}

pub struct DummyWindow;

impl HalWindow for DummyWindow {
  fn new(_settings: WindowSettings) -> Self {
    Self
  }

  fn title(&self) -> String {
    String::new()
  }
}
