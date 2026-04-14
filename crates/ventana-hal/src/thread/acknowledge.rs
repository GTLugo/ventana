use crossbeam_channel::{
  Receiver,
  Sender,
};

#[derive(Debug, Clone)]
pub struct AcknowledgementToken(Sender<()>);

impl AcknowledgementToken {
  pub fn new() -> (Self, Receiver<()>) {
    let (ack_tx, ack_rx) = crossbeam_channel::bounded(0);
    (Self(ack_tx), ack_rx)
  }

  pub fn send(self) {
    self.0.send(()).unwrap();
  }
}
