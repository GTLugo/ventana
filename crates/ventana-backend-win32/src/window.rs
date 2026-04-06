mod command;
mod iter;
mod state;
mod sync;
mod thread;

use {
  self::{
    command::Command,
    iter::Win32EventIterator,
    state::Internal,
  },
  ::win64::Handle,
  std::sync::Arc,
  ventana_hal::{
    dpi::{
      Position,
      Size,
    },
    error::RequestError,
    event::{
      Event,
      WindowEvent,
    },
    input::mouse::MouseButton,
    keyboard::{
      Code,
      KeyState,
    },
    monitor::BackendMonitor,
    settings::WindowSettings,
    types::{
      Flow,
      Stage,
    },
    window::{
      BackendEventIterator,
      BackendWindow,
      WindowId,
    },
  },
  win64::prelude::*,
};

pub struct Win32Window {
  hwnd: Window,
  internal: Arc<Internal>, // This is Arc so that it can be shared with the Window thread
}

impl Win32Window {
  pub fn new(settings: WindowSettings) -> Result<Self, RequestError> {
    set_process_dpi_awareness(DPIAwarenessContext::PerMonitorAwareV2);

    let internal = Internal::new(settings.clone());

    let (window_sender, window_receiver) = std::sync::mpsc::sync_channel(0);
    internal
      .thread
      .lock()
      .unwrap()
      .spawn(window_sender, internal.clone(), settings)?;

    log::trace!("Waiting to receive window handle back from window thread");

    let hwnd = window_receiver
      .recv()
      .expect("Failed to receive window back from window thread");

    log::trace!("Received window handle from window thread: `{hwnd:?}`");

    Ok(Self { hwnd, internal })
  }

  fn take_event(&self) -> Option<Event> {
    let flow = self.internal.state_lock().flow;
    if let Flow::Wait = flow {
      let no_events = self.internal.event_lock().is_none();
      if no_events {
        self.internal.sync.new_event.wait().unwrap();
      }
    }

    self.internal.event_lock().take().or(Some(Event::None))
  }
}

impl BackendWindow for Win32Window {
  fn id(&self) -> WindowId {
    WindowId::from_raw(self.hwnd.to_ptr() as usize)
  }

  fn raw_window_handle(&self) -> ventana_hal::raw_window_handle::RawWindowHandle {
    #[cfg(raw_window_handle_v5)]
    {
      let mut handle = ventana_hal::raw_window_handle::Win32WindowHandle::empty();
      handle.hwnd = self.hwnd.to_ptr();
      handle.hinstance = self.hwnd.instance().to_ptr();
      handle.into()
    }

    #[cfg(raw_window_handle_v6)]
    {
      let mut handle = ventana_hal::raw_window_handle::Win32WindowHandle::new(
        std::num::NonZeroIsize::new(self.hwnd.to_ptr() as isize).expect("window handle should not be zero"),
      );

      let hinstance = std::num::NonZeroIsize::new(self.hwnd.instance().to_ptr() as isize)
        .expect("instance handle should not be zero");
      handle.hinstance = Some(hinstance);
      handle.into()
    }
  }

  fn raw_display_handle(&self) -> ventana_hal::raw_window_handle::RawDisplayHandle {
    #[cfg(raw_window_handle_v5)]
    {
      let handle = ventana_hal::raw_window_handle::WindowsDisplayHandle::empty();
      handle.into()
    }

    #[cfg(raw_window_handle_v6)]
    {
      let handle = ventana_hal::raw_window_handle::WindowsDisplayHandle::new();
      handle.into()
    }
  }

  fn next_event(&self) -> Option<Event> {
    self.internal.sync.next_frame.signal().unwrap();

    let current_stage = self.internal.state_lock().stage;

    match current_stage {
      Stage::Setup => None,
      Stage::Quit => {
        self.internal.sync.skip_wait(true);
        self.internal.thread.lock().unwrap().join().unwrap();
        None
      },
      Stage::Looping | Stage::Closing => {
        let event = self.take_event();
        if let Some(Event::Window(WindowEvent::CloseRequest)) = event {
          let x = self.internal.state_lock().close_on_x;
          if x {
            self.close();
          }
        }
        event
      },
    }
  }

  fn iter<'w>(&'w self) -> Box<dyn BackendEventIterator<'w> + 'w> {
    self.internal.state_lock().stage = Stage::Looping;
    Box::new(Win32EventIterator::new(self))
  }

  fn monitor(&self) -> Arc<dyn BackendMonitor> {
    todo!()
  }

  fn close(&self) {
    if self.is_closing() {
      return; // already closing
    }

    log::trace!("[`{}`]: closing window", self.title());
    self.internal.state_lock().stage = Stage::Closing;
    Command::Destroy.post(self.hwnd);
  }

  fn is_closing(&self) -> bool {
    self.internal.state_lock().is_closing()
  }

  fn title(&self) -> String {
    self.hwnd.get_window_text().unwrap()
  }

  fn scale_factor(&self) -> f64 {
    self.hwnd.scale_factor()
  }

  fn inner_size(&self) -> Size {
    self.hwnd.inner_size()
  }

  fn outer_size(&self) -> Size {
    self.hwnd.outer_size()
  }

  fn inner_position(&self) -> Position {
    self.hwnd.inner_position()
  }

  fn outer_position(&self) -> Position {
    self.hwnd.outer_position()
  }

  fn key(&self, keycode: Code) -> KeyState {
    log::debug!("Checking {keycode:?}...");
    todo!()
  }

  fn mouse(&self, button: MouseButton) -> KeyState {
    log::debug!("Checking {button:?}...");
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
