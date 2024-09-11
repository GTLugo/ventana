use self::settings::WindowSettings;

pub mod position;
pub mod settings;
pub mod size;

pub trait Backend {
  /// For debugging purposes
  fn name() -> &'static str
  where
    Self: Sized;
  /// Use this to create the crate-specific window object
  fn create_window(&self, settings: WindowSettings) -> Box<dyn Window>;
}

pub trait Window {
  fn new(settings: WindowSettings) -> Self
  where
    Self: Sized;

  fn title(&self) -> String;
}

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
