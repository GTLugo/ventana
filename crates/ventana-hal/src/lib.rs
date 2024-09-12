use self::{position::Position, settings::WindowSettings, size::Size};

pub mod position;
pub mod settings;
pub mod size;

// #[cfg(feature = "derive")]
// pub use ventana_derive::Backend;

pub trait Backend {
  /// Use this to create the crate-specific window object
  fn create_window(&self, settings: WindowSettings) -> Box<dyn Window>;
}

// Maybe split this into things like "BasicWindow" "ResizableWindow" "MoveableWindow" etc
pub trait Window: Send + Sync {
  // fn id(&self) -> WindowId;

  fn title(&self) -> String;

  fn size(&self) -> Size;

  fn position(&self) -> Position;
}

// pub trait CreateWindow<W: Window> {
//   fn create_window(settings: WindowSettings) -> impl Window {
//     W::new(settings)
//   }
// }

// pub enum Platform<T: Backend = ()> {
//   Win32,
//   X11,
//   Custom(T),
// }

// impl Backend for () {
//   fn name(&self) -> &'static str {
//     unimplemented!()
//   }

//   fn create_window(&self, settings: WindowSettings) -> Box<dyn Window>
//   where
//     Self: Sized,
//   {
//     unimplemented!()
//   }
// }

// WIP. Syntax errors due to me changing my mind on it while working on it
// #[macro_export]
// macro_rules! backends {
//     (box $backend:ty) => {{
//       Box::new($backend)
//     }};
//     (box $backend:ty, $($backends:ty),*) => {
//         $crate::backends!(box $backend),
//         $crate::backends!(box $($backends),*)
//     };
//     ($($backends:ty),+) => {{

//       vec![
//         $crate::backends!(box $($backends),+)
//       ]
//     }};
// }
