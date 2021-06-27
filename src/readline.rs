use std::ffi::{CStr, CString};
use std::str::Utf8Error;

pub fn prompt<S: AsRef<str>>(prompt: S) -> CString {
  CString::new(prompt.as_ref()).unwrap()
}

pub fn readline(prompt: &CStr) -> Result<Option<String>, Utf8Error> {
  let p = prompt.as_ptr();

  let cstr_ptr = unsafe { sys::readline(p) };
  if cstr_ptr.is_null() {
    Ok(None)
  } else {
    let cstr = unsafe { CStr::from_ptr(cstr_ptr) };
    let r = cstr.to_str().map(|s| Some(String::from(s)));
    unsafe { sys::free(cstr_ptr as _) };
    r
  }
}

mod sys {
  use std::os::raw::c_char;
  use std::ffi::c_void;
  #[link(name="readline")]
  extern "C" {
    pub fn free(ptr: *mut c_void);
    pub fn readline(prompt: *const c_char) -> *mut c_char;
  }
}
