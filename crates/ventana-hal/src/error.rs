// Taken directly from Winit under the Apache License 2.0 license https://github.com/rust-windowing/winit/blob/master/winit-core/src/error.rs

use {
  smol_str::SmolStr,
  std::fmt::{
    self,
    Display,
  },
};

/// A general error that may occur during a request to the windowing system.
#[derive(thiserror::Error, Debug)]
#[non_exhaustive]
pub enum RequestError {
  /// The request is not supported.
  #[error("The request is not supported. `{0}`")]
  NotSupported(SmolStr),
  #[error("The request was ignored by the OS")]
  Ignored,
  /// Got unspecified OS specific error during the request.
  #[error("OS specific error. `{0}`")]
  Os(#[from] OsError),
}

impl RequestError {
  pub fn not_supported(message: impl Into<SmolStr>) -> Self {
    Self::NotSupported(message.into())
  }
}

/// Unclassified error from the OS.
#[derive(thiserror::Error, Debug)]
pub struct OsError {
  line: u32,
  file: &'static str,
  error: Box<dyn std::error::Error + Send + Sync + 'static>,
}

impl OsError {
  pub fn new(
    line: u32,
    file: &'static str,
    error: impl Into<Box<dyn std::error::Error + Send + Sync + 'static>>,
  ) -> Self {
    Self {
      line,
      file,
      error: error.into(),
    }
  }
}

impl Display for OsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.pad(&format!("os error at {}:{}: {}", self.file, self.line, self.error))
  }
}

pub trait MapToOSError<T, E> {
  fn map_to_os_err(self) -> Result<T, OsError>
  where
    Self: Sized;

  fn request_error(self) -> Result<T, RequestError>
  where
    Self: Sized;
}

impl<T, E> MapToOSError<T, E> for Result<T, E>
where
  E: std::error::Error + Send + Sync + 'static,
{
  fn map_to_os_err(self) -> Result<T, OsError>
  where
    Self: Sized,
  {
    match self {
      Ok(t) => Ok(t),
      Err(e) => Err(crate::os_error!(e)),
    }
  }

  fn request_error(self) -> Result<T, RequestError>
  where
    Self: Sized,
  {
    Ok(self.map_to_os_err()?)
  }
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! os_error {
  ($error:expr) => {{ $crate::error::OsError::new(line!(), file!(), $error) }};
}

#[allow(unused_macros)]
#[macro_export]
macro_rules! os_error_fmt {
  ($error:expr) => {{ $crate::os_error!(format!($error)) }};
}
