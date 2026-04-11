use {
  super::thread::WindowThread,
  crate::window::{
    command::CommandEnvelope,
    sync::{
      AcknowledgementToken,
      EventEnvelope,
      ResponseEnvelope,
      WindowToMain,
    },
  },
  crossbeam_channel::{
    Receiver,
    Sender,
  },
  std::sync::{
    Arc,
    Mutex,
    MutexGuard,
    atomic::{
      AtomicBool,
      Ordering,
    },
  },
  ventana_hal::{
    error::RequestError,
    event::Event,
    settings::WindowSettings,
    types::Flow,
  },
  win64::user::Window,
};

pub(crate) struct SharedInternal {
  is_ready: AtomicBool,
  state: Mutex<State>,

  msg_tx: Sender<WindowToMain>,
  cmd_rx: Receiver<CommandEnvelope>,

  thread: Mutex<WindowThread>,
}

impl Drop for SharedInternal {
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

impl SharedInternal {
  pub fn new(settings: WindowSettings, msg_tx: Sender<WindowToMain>, cmd_rx: Receiver<CommandEnvelope>) -> Arc<Self> {
    Arc::new(Self {
      is_ready: AtomicBool::new(false),
      state: Mutex::new(State::new(settings.clone())),
      // event: Mutex::new(None),
      // sync: SyncData::new(),
      msg_tx,
      cmd_rx,
      thread: Mutex::new(WindowThread::new()),
    })
  }

  pub fn spawn_thread(self: &Arc<Self>, settings: WindowSettings) -> Result<Window, RequestError> {
    let mut thread = self.thread_lock();
    let hwnd = thread.spawn(self.clone(), settings)?;

    Ok(hwnd)
  }

  pub fn set_ready(&self) {
    self.is_ready.store(true, Ordering::Release)
  }

  pub fn is_ready(&self) -> bool {
    self.is_ready.load(Ordering::Acquire)
  }

  pub fn should_close(&self) -> bool {
    !self.state_lock().is_running
  }

  // pub fn destroy(&self, hwnd: Window) {
  //     Command::Destroy.post(hwnd);
  //   if self.state_lock().stage != Stage::Destroyed {
  //     self.state_lock().stage = Stage::Destroyed;
  //     Command::Destroy.post(hwnd);
  //   }
  // }

  // pub fn event_lock(&self) -> MutexGuard<'_, Option<Event>> {
  //   self.event.lock().unwrap()
  // }

  pub fn state_lock(&self) -> MutexGuard<'_, State> {
    self.state.lock().unwrap()
  }

  pub fn thread_lock(&self) -> MutexGuard<'_, WindowThread> {
    self.thread.lock().unwrap()
  }

  pub fn are_commands_pending(&self) -> bool {
    !self.cmd_rx.is_empty()
  }

  pub fn receive_command(&self) -> Option<CommandEnvelope> {
    self.cmd_rx.try_recv().ok()
  }

  pub fn send_response(&self, response: ResponseEnvelope) {
    self
      .msg_tx
      .try_send(WindowToMain::CommandResponse(response))
      .expect("failed to send response");
  }

  pub fn send_event(&self, event: Event, should_block: bool) {
    let (ack, receiver) = if should_block {
      let (tx, rx) = AcknowledgementToken::new();
      (Some(tx), Some(rx))
    } else {
      (None, None)
    };
    self
      .msg_tx
      .try_send(WindowToMain::Event(EventEnvelope { event, ack }))
      .ok();
    if let Some(receiver) = receiver {
      receiver.recv().expect("failed to receive acknowledgement");
    }
  }
}

pub struct State {
  pub(crate) is_running: bool,
  pub(crate) flow: Flow,
  pub(crate) close_on_x: bool,
}

impl State {
  fn new(settings: WindowSettings) -> Self {
    Self {
      is_running: true,
      flow: settings.flow,
      close_on_x: settings.close_on_x,
    }
  }
}
