// use ventana_hal::{
//   message::Message, position::Position, settings::WindowSettings, size::Size, WindowProvider, Window as HalWindow,
// };

// pub struct WaylandBackend;

// impl WindowProvider for WaylandBackend {
//   fn create_window(&self, settings: WindowSettings) -> Box<dyn HalWindow> {
//     Box::new(Window { settings })
//   }
// }

// pub struct Window {
//   settings: WindowSettings,
// }

// impl HalWindow for Window {
//   fn next(&self) -> Option<Message> {
//     None
//   }

//   fn title(&self) -> String {
//     self.settings.title.clone()
//   }

//   fn size(&self) -> Size {
//     self.settings.size
//   }

//   fn position(&self) -> Position {
//     self.settings.position
//   }
// }
