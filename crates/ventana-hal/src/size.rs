use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Size {
  Logical(LogicalSize),
  Physical(PhysicalSize),
}

impl Size {
  pub fn new(size: impl Into<Self>) -> Self {
    size.into()
  }

  pub fn as_logical(&self, scale_factor: f64) -> LogicalSize {
    match *self {
      Size::Logical(size) => size,
      Size::Physical(size) => size.as_logical(scale_factor),
    }
  }

  pub fn as_physical(&self, scale_factor: f64) -> PhysicalSize {
    match *self {
      Size::Logical(size) => size.as_physical(scale_factor),
      Size::Physical(size) => size,
    }
  }
}

impl From<LogicalSize> for Size {
  fn from(val: LogicalSize) -> Self {
    Self::Logical(val)
  }
}

impl From<PhysicalSize> for Size {
  fn from(val: PhysicalSize) -> Self {
    Self::Physical(val)
  }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LogicalSize {
  pub width: f64,
  pub height: f64,
}

impl LogicalSize {
  pub fn new(width: f64, height: f64) -> Self {
    Self { width, height }
  }

  pub fn as_physical(&self, scale_factor: f64) -> PhysicalSize {
    PhysicalSize::new(self.width.round() as u32, self.height.round() as u32)
      * scale_factor
  }

  pub fn is_any_positive(&self) -> bool {
    self.width > 0.0 || self.height > 0.0
  }

  pub fn is_all_positive(&self) -> bool {
    self.width > 0.0 && self.height > 0.0
  }

  pub fn is_any_negative(&self) -> bool {
    self.width < 0.0 || self.height < 0.0
  }

  pub fn is_all_negative(&self) -> bool {
    self.width < 0.0 && self.height < 0.0
  }

  pub fn is_any_zero(&self) -> bool {
    self.width == 0.0 || self.height == 0.0
  }

  pub fn is_all_zero(&self) -> bool {
    self.width == 0.0 && self.height == 0.0
  }
}

impl Div<f64> for LogicalSize {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    Self {
      height: (self.height / rhs).round(),
      width: (self.width / rhs).round(),
    }
  }
}

impl Mul<f64> for LogicalSize {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      height: (self.height * rhs).round(),
      width: (self.width * rhs).round(),
    }
  }
}

impl From<LogicalSize> for (f64, f64) {
  fn from(val: LogicalSize) -> Self {
    (val.width, val.height)
  }
}

impl From<LogicalSize> for [f64; 2] {
  fn from(val: LogicalSize) -> Self {
    [val.width, val.height]
  }
}

impl From<(f64, f64)> for LogicalSize {
  fn from(value: (f64, f64)) -> Self {
    Self {
      width: value.0,
      height: value.1,
    }
  }
}

impl From<[f64; 2]> for LogicalSize {
  fn from(value: [f64; 2]) -> Self {
    Self {
      width: value[0],
      height: value[1],
    }
  }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PhysicalSize {
  pub width: u32,
  pub height: u32,
}

impl PhysicalSize {
  pub fn new(width: u32, height: u32) -> Self {
    Self { width, height }
  }

  pub fn as_logical(&self, scale_factor: f64) -> LogicalSize {
    LogicalSize::new(self.width as f64, self.height as f64) / scale_factor
  }

  pub fn is_any_zero(&self) -> bool {
    self.width == 0 || self.height == 0
  }

  pub fn is_all_zero(&self) -> bool {
    self.width == 0 && self.height == 0
  }
}

impl Div<f64> for PhysicalSize {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    Self {
      height: (self.height as f64 / rhs).round() as u32,
      width: (self.width as f64 / rhs).round() as u32,
    }
  }
}

impl Mul<f64> for PhysicalSize {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      height: (self.height as f64 * rhs).trunc() as u32,
      width: (self.width as f64 * rhs).trunc() as u32,
    }
  }
}

impl From<PhysicalSize> for (u32, u32) {
  fn from(val: PhysicalSize) -> Self {
    (val.width, val.height)
  }
}

impl From<PhysicalSize> for (i32, i32) {
  fn from(val: PhysicalSize) -> Self {
    (val.width as i32, val.height as i32)
  }
}

impl From<PhysicalSize> for [u32; 2] {
  fn from(val: PhysicalSize) -> Self {
    [val.width, val.height]
  }
}

impl From<PhysicalSize> for [i32; 2] {
  fn from(val: PhysicalSize) -> Self {
    [val.width as i32, val.height as i32]
  }
}

impl From<(u32, u32)> for PhysicalSize {
  fn from(value: (u32, u32)) -> Self {
    Self {
      width: value.0,
      height: value.1,
    }
  }
}

impl From<[u32; 2]> for PhysicalSize {
  fn from(value: [u32; 2]) -> Self {
    Self {
      width: value[0],
      height: value[1],
    }
  }
}