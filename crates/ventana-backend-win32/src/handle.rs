pub mod instance;
pub mod window;

pub trait Win32Type {
  type Type;

  fn to_win32(&self) -> Self::Type;
}

pub trait Handle: Sized {
  // fn to_any(&self) -> impl core::any::Any;
  fn as_ptr(&self) -> *mut core::ffi::c_void;
  /// # Safety
  /// Ensure that the ptr is a valid object before conversion
  unsafe fn from_ptr(ptr: *mut core::ffi::c_void) -> Self;
  fn is_valid(&self) -> bool;
}
