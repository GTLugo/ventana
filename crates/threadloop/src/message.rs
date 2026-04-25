use {
  crate::{
    Result,
    signal::AcknowledgeSignal,
  },
  std::{
    collections::HashMap,
    sync::{
      Condvar,
      Mutex,
      atomic::{
        AtomicU64,
        Ordering,
      },
    },
  },
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct Id(u64);

impl Id {
  fn next() -> Self {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    Self(COUNTER.fetch_add(1, Ordering::Relaxed))
  }
}

#[derive(Debug)]
pub struct Envelope<M> {
  pub message: M,
  pub ack: Option<AcknowledgeSignal>,
}

#[derive(Debug)]
pub enum ServerToClient<E, Ready> {
  Ready(Result<Ready>),
  Event(Envelope<E>),
  Stop,
}

#[derive(Debug)]
pub enum ClientToServer<R, Start> {
  Start { id: Id, params: Start },
  Request { id: Id, request: R },
  Stop { id: Id },
}

impl<R, Start> ClientToServer<R, Start> {
  pub fn start(params: Start) -> Self {
    Self::Start { id: Id::next(), params }
  }

  pub fn request(request: R) -> Self {
    Self::Request {
      id: Id::next(),
      request,
    }
  }

  pub fn stop() -> Self {
    Self::Stop { id: Id::next() }
  }

  pub fn id(&self) -> Id {
    match self {
      ClientToServer::Start { id, .. } | ClientToServer::Request { id, .. } | ClientToServer::Stop { id } => *id,
    }
  }
}

#[derive(Debug)]
pub(crate) struct ResponseStore<R> {
  map: Mutex<HashMap<Id, R>>,
  con: Condvar,
}

impl<R> ResponseStore<R> {
  pub fn new() -> Self {
    Self {
      map: Default::default(),
      con: Default::default(),
    }
  }

  pub fn insert_and_notify(&self, id: Id, response: R) {
    let mut g = self.map.lock().unwrap();
    g.insert(id, response);
    self.con.notify_all();
  }

  pub fn wait_and_take(&self, id: &Id) -> Option<R> {
    let mut map = self
      .con
      .wait_while(self.map.lock().unwrap(), |map| !map.contains_key(id))
      .unwrap();
    map.remove(id)
  }

  // pub fn try_take(&self, id: &Id) -> Option<R> {
  //   self.map.lock().unwrap().remove(id)
  // }
}
