use std::ops::{Div, Mul};

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Position {
  Logical(LogicalPosition),
  Physical(PhysicalPosition),
}

impl Position {
  pub fn new(position: impl Into<Self>) -> Self {
    position.into()
  }

  pub fn as_logical(&self, scale_factor: f64) -> LogicalPosition {
    match *self {
      Position::Logical(position) => position,
      Position::Physical(position) => position.as_logical(scale_factor),
    }
  }

  pub fn as_physical(&self, scale_factor: f64) -> PhysicalPosition {
    match *self {
      Position::Logical(position) => position.as_physical(scale_factor),
      Position::Physical(position) => position,
    }
  }
}

impl From<LogicalPosition> for Position {
  fn from(val: LogicalPosition) -> Self {
    Self::Logical(val)
  }
}

impl From<(f64, f64)> for Position {
  fn from(val: (f64, f64)) -> Self {
    Self::Logical(val.into())
  }
}

impl From<[f64; 2]> for Position {
  fn from(val: [f64; 2]) -> Self {
    Self::Logical(val.into())
  }
}

impl From<PhysicalPosition> for Position {
  fn from(val: PhysicalPosition) -> Self {
    Self::Physical(val)
  }
}

impl From<(i32, i32)> for Position {
  fn from(val: (i32, i32)) -> Self {
    Self::Physical(val.into())
  }
}

impl From<[i32; 2]> for Position {
  fn from(val: [i32; 2]) -> Self {
    Self::Physical(val.into())
  }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct LogicalPosition {
  pub x: f64,
  pub y: f64,
}

impl LogicalPosition {
  pub fn new(x: f64, y: f64) -> Self {
    Self { x, y }
  }

  pub fn as_physical(&self, scale_factor: f64) -> PhysicalPosition {
    PhysicalPosition::new(self.x.round() as i32, self.y.round() as i32) * scale_factor
  }

  pub fn is_positive(&self) -> bool {
    self.x > 0.0 && self.y > 0.0
  }

  pub fn is_negative(&self) -> bool {
    self.x < 0.0 && self.y < 0.0
  }

  pub fn is_zero(&self) -> bool {
    self.x == 0.0 && self.y == 0.0
  }
}

impl Div<f64> for LogicalPosition {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    Self {
      y: (self.y / rhs).round(),
      x: (self.x / rhs).round(),
    }
  }
}

impl Mul<f64> for LogicalPosition {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      y: (self.y * rhs).round(),
      x: (self.x * rhs).round(),
    }
  }
}

impl From<LogicalPosition> for (f64, f64) {
  fn from(val: LogicalPosition) -> Self {
    (val.x, val.y)
  }
}

impl From<LogicalPosition> for [f64; 2] {
  fn from(val: LogicalPosition) -> Self {
    [val.x, val.y]
  }
}

impl From<(f64, f64)> for LogicalPosition {
  fn from(value: (f64, f64)) -> Self {
    Self {
      x: value.0,
      y: value.1,
    }
  }
}

impl From<[f64; 2]> for LogicalPosition {
  fn from(value: [f64; 2]) -> Self {
    Self {
      x: value[0],
      y: value[1],
    }
  }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Hash)]
pub struct PhysicalPosition {
  pub x: i32,
  pub y: i32,
}

impl PhysicalPosition {
  pub fn new(x: i32, y: i32) -> Self {
    Self { x, y }
  }

  pub fn as_logical(&self, scale_factor: f64) -> LogicalPosition {
    LogicalPosition::new(self.x as f64, self.y as f64) / scale_factor
  }

  pub fn is_positive(&self) -> bool {
    self.x > 0 && self.y > 0
  }

  pub fn is_negative(&self) -> bool {
    self.x < 0 && self.y < 0
  }

  pub fn is_zero(&self) -> bool {
    self.x == 0 && self.y == 0
  }
}

impl Div<f64> for PhysicalPosition {
  type Output = Self;

  fn div(self, rhs: f64) -> Self::Output {
    Self {
      y: (self.y as f64 / rhs).round() as i32,
      x: (self.x as f64 / rhs).round() as i32,
    }
  }
}

impl Mul<f64> for PhysicalPosition {
  type Output = Self;

  fn mul(self, rhs: f64) -> Self::Output {
    Self {
      y: (self.y as f64 * rhs).trunc() as i32,
      x: (self.x as f64 * rhs).trunc() as i32,
    }
  }
}

impl From<PhysicalPosition> for (u32, u32) {
  fn from(val: PhysicalPosition) -> Self {
    (val.x as u32, val.y as u32)
  }
}

impl From<PhysicalPosition> for (i32, i32) {
  fn from(val: PhysicalPosition) -> Self {
    (val.x, val.y)
  }
}

impl From<PhysicalPosition> for [u32; 2] {
  fn from(val: PhysicalPosition) -> Self {
    [val.x as u32, val.y as u32]
  }
}

impl From<PhysicalPosition> for [i32; 2] {
  fn from(val: PhysicalPosition) -> Self {
    [val.x, val.y]
  }
}

impl From<(i32, i32)> for PhysicalPosition {
  fn from(value: (i32, i32)) -> Self {
    Self {
      x: value.0,
      y: value.1,
    }
  }
}

impl From<[i32; 2]> for PhysicalPosition {
  fn from(value: [i32; 2]) -> Self {
    Self {
      x: value[0],
      y: value[1],
    }
  }
}