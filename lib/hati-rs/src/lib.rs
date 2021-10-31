use hati_c as ffi;

use std::ffi::CStr;
use std::str::Utf8Error;

pub fn hello_world() -> Result<&'static str, Utf8Error> {
  unsafe { CStr::from_ptr(ffi::hello_world()) }.to_str()
}