use std::ptr::NonNull;

use windows::{
  core::{HSTRING, PCWSTR},
  Win32::{
    Foundation::{HINSTANCE, HMODULE},
    System::LibraryLoader::{
      GetModuleHandleExW, GetModuleHandleW, GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
      GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT,
    },
  },
};

use super::{Handle, Win32Type};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct InstanceId(Option<NonNull<usize>>);

impl Default for InstanceId {
  fn default() -> Self {
    Self::get_current()
  }
}

impl Win32Type for InstanceId {
  type Type = HINSTANCE;

  fn to_win32(&self) -> Self::Type {
    (*self).into()
  }
}

impl InstanceId {
  // #[inline]
  // pub fn get_exe() -> Handle<Self> {
  //   Self::get(None::<&str>).unwrap() // shouldn't fail, right?
  // }

  pub fn get(module: Option<impl Into<String>>) -> Result<InstanceId, windows::core::Error> {
    unsafe {
      GetModuleHandleW(
        module
          .map(Into::into)
          .map(HSTRING::from)
          .map_or_else(PCWSTR::null, |n| PCWSTR(n.as_ptr())),
      )
    }
    .map(|value| HINSTANCE(value.0).into())
  }

  pub fn get_current() -> InstanceId {
    // https://stackoverflow.com/questions/557081/how-do-i-get-the-hmodule-for-the-currently-executing-code
    let mut h_module = HMODULE::default();

    if let Err(e) = unsafe {
      GetModuleHandleExW(
        GET_MODULE_HANDLE_EX_FLAG_UNCHANGED_REFCOUNT | GET_MODULE_HANDLE_EX_FLAG_FROM_ADDRESS,
        PCWSTR::from_raw(Self::get_current as *const u16),
        &mut h_module,
      )
    } {
      eprintln!("get_current_module error: {e}");
    }

    HINSTANCE(h_module.0).into()
  }
}

impl From<InstanceId> for HINSTANCE {
  fn from(value: InstanceId) -> Self {
    Self(value.as_ptr())
  }
}

impl From<HINSTANCE> for InstanceId {
  fn from(value: HINSTANCE) -> Self {
    unsafe { Self::from_ptr(value.0) }
  }
}

impl Handle for InstanceId {
  fn as_ptr(&self) -> *mut core::ffi::c_void {
    self.0.map_or(core::ptr::null_mut(), |ptr| ptr.as_ptr().cast())
  }

  unsafe fn from_ptr(ptr: *mut core::ffi::c_void) -> Self {
    let ptr: *mut usize = ptr.cast();
    Self(match ptr.is_null() {
      true => None,
      false => Some(unsafe { NonNull::new_unchecked(ptr) }),
    })
  }

  fn is_valid(&self) -> bool {
    self.0.is_some()
  }
}
