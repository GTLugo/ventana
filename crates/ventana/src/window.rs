use std::ops::Deref;

use ventana_hal::{
  position::Position, settings::WindowSettings, size::Size, Backend as HalBackend, Window as HalWindow,
};

use crate::backend;

pub struct Window(Box<dyn HalWindow>);

impl Window {
  pub fn builder() -> WindowBuilder {
    WindowBuilder::new()
  }

  // pub fn with_backend<T: HalBackend + 'static>(settings: &WindowSettings) -> Self {
  //   // let backend = (backend.0)();
  //   // eprintln!("Using backend: `{}`", backend.0.name());
  //   Self(Box::new(T::create_window(settings.clone())))
  // }

  pub fn new(backend: &Backend, settings: &WindowSettings) -> Self {
    Self(backend.create_window(settings.clone()))
  }

  pub fn title(&self) -> String {
    self.0.title()
  }

  pub fn size(&self) -> Size {
    self.0.size()
  }

  pub fn position(&self) -> Position {
    self.0.position()
  }
}

pub struct Backend(Box<dyn HalBackend>);

impl Default for Backend {
  fn default() -> Self {
    #[allow(unreachable_code)]
    fn pick() -> Box<dyn HalBackend> {
      #[cfg(windows_platform)]
      return Box::new(backend::win32::Win32);
      #[cfg(x11_platform)]
      return Box::new(backend::x11::X11Backend);
      panic!("No valid default backend selected for ventana. Check feature flags or try manually selecting a compatible backend.");
    }

    Self(pick())
  }
}

impl<T: HalBackend + 'static> From<T> for Backend {
  fn from(value: T) -> Self {
    Self(Box::new(value))
  }
}

impl Deref for Backend {
  type Target = dyn HalBackend;

  fn deref(&self) -> &Self::Target {
    self.0.as_ref()
  }
}

// impl Backend {
//   pub fn select(selector: impl Fn() -> Box<dyn HalBackend> + 'static) -> Self {
//     Self(Box::new(selector))
//   }
// }

pub struct WindowBuilder {
  backend: Backend,
  settings: WindowSettings,
}

impl Default for WindowBuilder {
  fn default() -> Self {
    Self::new()
  }
}

impl WindowBuilder {
  pub fn new() -> Self {
    Self {
      backend: Backend::default(),
      settings: WindowSettings::default(),
    }
  }

  pub fn with_backend(&mut self, backend: impl Into<Backend>) -> &mut Self {
    self.backend = backend.into();
    self
  }

  pub fn build(&self) -> Window {
    Window::new(&self.backend, &self.settings)
  }
}

// pub struct DefaultBackend;

// impl HalBackend for DefaultBackend {
//   type Window = Box<dyn HalWindow>;

//   #[allow(unreachable_code)]
//   fn create_window(settings: WindowSettings) -> Self::Window {
//     #[cfg(windows_platform)]
//     return Box::new(backend::win32::Win32::create_window(settings));
//     #[cfg(x11_platform)]
//     todo!("x11 is todo after win32");
//     panic!("No valid default backend selected for ventana. Check feature flags or try manually selecting a compatible backend.");
//   }
// }
