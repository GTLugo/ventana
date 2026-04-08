use {
  super::{
    sync::SyncData,
    thread::WindowThread,
  },
  std::sync::{
    Arc,
    Mutex,
    MutexGuard,
  },
  ventana_hal::{
    event::Event,
    settings::WindowSettings,
    types::{
      Flow,
      Stage,
    },
  },
};

pub(crate) struct Internal {
  pub state: Mutex<State>,
  pub event: Mutex<Option<Event>>,
  pub sync: SyncData,
  pub thread: Mutex<WindowThread>,
}

impl Drop for Internal {
  fn drop(&mut self) {
    // let title = self.data_lock().title.clone();

    // if self.state_lock().stage == Stage::Destroyed {
    //   return;
    // } else {
    //   self.state_lock().stage = Stage::Destroyed;
    // }

    // tracing::trace!("[`{}`]: destroying window", title);

    // tracing::trace!("[`{}`]: unregistering window class", title);
    // unsafe { UnregisterClassW(PCWSTR(self.class_atom as *const u16), self.hinstance) }
    //   .unwrap();

    // tracing::trace!("[`{}`]: destroyed window", title);
  }
}

impl Internal {
  pub fn new(settings: WindowSettings) -> Arc<Self> {
    Arc::new(Self {
      state: Mutex::new(State::new(settings)),
      event: Mutex::new(None),
      sync: SyncData::new(),
      thread: Mutex::new(WindowThread::new()),
    })
  }

  // pub fn destroy(&self, hwnd: Window) {
  //     Command::Destroy.post(hwnd);
  //   if self.state_lock().stage != Stage::Destroyed {
  //     self.state_lock().stage = Stage::Destroyed;
  //     Command::Destroy.post(hwnd);
  //   }
  // }

  pub fn event_lock(&self) -> MutexGuard<'_, Option<Event>> {
    self.event.lock().unwrap()
  }

  pub fn state_lock(&self) -> MutexGuard<'_, State> {
    self.state.lock().unwrap()
  }

  pub fn send_event_to_main(&self, event: Event) {
    let should_wait = self.event.lock().unwrap().is_some();
    if should_wait {
      self.sync.next_frame.wait().unwrap();
    }

    self.event.lock().unwrap().replace(event);
    self.sync.new_event.signal().unwrap();

    // TODO: try inverting these locks so that they don't lock unless the main thread tells them to lock.

    self.sync.next_frame.wait().unwrap(); // This is problematic since it will cause a deadlock if the main thread sends any messages to the window thread
  }
}

pub struct State {
  pub(crate) stage: Stage,
  pub(crate) flow: Flow,
  pub(crate) close_on_x: bool,
}

impl State {
  fn new(settings: WindowSettings) -> Self {
    Self {
      stage: Stage::Setup,
      flow: settings.flow,
      close_on_x: settings.close_on_x,
    }
  }

  pub fn is_closing(&self) -> bool {
    matches!(self.stage, Stage::Closing | Stage::Quit)
  }
}
