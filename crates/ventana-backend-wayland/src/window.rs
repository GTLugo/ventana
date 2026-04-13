#![cfg(all(
  unix,
  not(any(
    target_os = "redox",
    target_family = "wasm",
    target_os = "android",
    target_vendor = "apple"
  ))
))]

// mod state;

// use {
//   self::{
//     state::{
//       SharedState,
//       WaylandState,
//     },
//   },
//   sctk::{
//     reexports::{
//       calloop::{
//         EventLoop,
//         LoopSignal,
//       },
//       calloop_wayland_source::WaylandSource,
//       client::{
//         Connection,
//         globals::{
//           self,
//         },
//       },
//     },
//     shell::xdg::window::Window,
//   },
//   std::sync::{
//     Arc,
//     Mutex,
//     MutexGuard,
//   },
//   synchronize::Signal,
//   ventana_hal::{
//     dpi::{
//       PhysicalPosition,
//       PhysicalSize,
//     },
//     error::{
//       MapToOSError,
//       RequestError,
//     },
//     event::{
//       Event,
//       WindowEvent,
//     },
//     settings::WindowSettings,
//     types::Flow,
//     window::{
//       BackendWindow,
//       WindowId,
//     },
//   },
// };

// pub struct CreateInfo {
//   pub id: WindowId,
//   pub window: Window,
//   pub loop_signal: LoopSignal,
// }

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
    mouse::{
      button::MouseButton,
      state::ButtonState,
    },
    window::{
      BackendWindow,
      WindowId,
    },
  },
};

pub struct WaylandWindow;

// pub struct WaylandWindow {
//   id: WindowId,
//   window: Window,
//   state: Arc<Mutex<SharedState>>,
//   loop_signal: LoopSignal,
//   event_signal: Signal,
//   iteration_signal: Signal,
// }

// impl WaylandWindow {
//   pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
//     // I can't find any decent beginner materials on how to actually make a Wayland window and read events,
//     // so this is HEAVILY based on Winit. If there are any quirks, it's probably because of me
//     // trying to warp Winit's implementation.

//     // let (event_tx, event_rx) = std::sync::mpsc::sync_channel(0);
//     let event_signal = Signal::new();
//     let iteration_signal = Signal::new();
//     let state = Arc::new(Mutex::new(SharedState {
//       id: WindowId::from_raw(0),
//       width: 0,
//       height: 0,
//       should_exit: false,
//       event: None,
//       event_signal: event_signal.clone(),
//       iteration_signal: iteration_signal.clone(),
//       flow: settings.flow,
//       close_on_x: settings.close_on_x,
//     }));
//     let shared_state = state.clone();
//     let shared_settings = settings.clone();

//     let connection = Connection::connect_to_env().map_to_os_err()?;
//     let (globals, event_queue) = globals::registry_queue_init(&connection).map_to_os_err()?;
//     let queue_handle = event_queue.handle();

//     let (info_tx, info_rx) = std::sync::mpsc::sync_channel(0);

//     std::thread::Builder::new()
//       .name("window".into())
//       .spawn(move || -> Result<(), RequestError> {
//         let mut event_loop: EventLoop<WaylandState> = EventLoop::try_new().map_to_os_err()?;

//         WaylandSource::new(connection.clone(), event_queue)
//           .insert(event_loop.handle())
//           .map_to_os_err()?;

//         let mut wayland_state =
//           WaylandState::new(&globals, &queue_handle, shared_state, event_loop.handle(), shared_settings)?;

//         info_tx
//           .send(CreateInfo {
//             id: wayland_state.id(),
//             window: wayland_state.window(),
//             loop_signal: event_loop.get_signal(),
//           })
//           .map_to_os_err()?;

//         loop {
//           if let Err(error) = event_loop.dispatch(None, &mut wayland_state) {
//             log::error!("{error}");
//           }

//           if wayland_state.state_lock().should_exit {
//             break Ok(());
//           }
//         }
//       })
//       .map_to_os_err()?;

//     let CreateInfo {
//       id,
//       window,
//       loop_signal,
//     } = info_rx.recv().map_to_os_err()?;

//     Ok(Self {
//       id,
//       window,
//       state,
//       loop_signal,
//       event_signal,
//       iteration_signal,
//     })
//   }

//   pub fn state_lock(&self) -> MutexGuard<'_, SharedState> {
//     self.state.lock().unwrap()
//   }

//   fn take_event(&self) -> Option<Event> {
//     let flow = self.state_lock().flow;
//     if let Flow::Wait = flow {
//       let no_events = self.state_lock().event.is_none();
//       if no_events {
//         self.event_signal.wait().unwrap();
//       }
//     }

//     self.state_lock().event.take().or(Some(Event::None))
//   }
// }

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

  fn title(&self) -> String {
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
