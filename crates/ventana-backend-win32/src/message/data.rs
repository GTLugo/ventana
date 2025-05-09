use windows::Win32::Foundation::{LPARAM, WPARAM};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MessageData {
  pub w: usize,
  pub l: isize,
}

impl MessageData {
  pub fn w_param(&self) -> WPARAM {
    WPARAM(self.w)
  }

  pub fn l_param(&self) -> LPARAM {
    LPARAM(self.l)
  }
}
