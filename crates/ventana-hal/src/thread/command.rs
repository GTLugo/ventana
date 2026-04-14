// #[derive(Debug)]
// pub enum ThreadCommand<Command> {
//   Command(CommandEnvelope<Command>),
//   Stop,
// }

use std::sync::atomic::{
  AtomicU64,
  Ordering,
};

#[derive(Debug, Clone)]
pub struct CommandEnvelope<Command> {
  pub id: CommandId,
  pub command: Command,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Ord, PartialOrd, Eq, Hash)]
pub struct CommandId(u64);

impl CommandId {
  pub fn next() -> Self {
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    Self(COUNTER.fetch_add(1, Ordering::Relaxed))
  }
}
