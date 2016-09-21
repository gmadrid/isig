#![allow(improper_ctypes, non_camel_case_types, non_upper_case_globals)]

use core_foundation::base::{CFAllocatorRef, CFIndexConvertible, TCFType};
use core_foundation_sys::base::{CFIndex, CFRelease, kCFAllocatorDefault};
use core_foundation_sys::data::*;
use std::mem;
use std::ptr;
use std::slice;

#[repr(C)]
pub struct __CFMutableData;

pub type CFMutableDataRef = *const __CFMutableData;

pub struct CFMutableData(CFMutableDataRef);

impl Drop for CFMutableData {
  fn drop(&mut self) {
    unsafe { CFRelease(self.as_CFTypeRef()) }
  }
}

impl_TCFType!(CFMutableData, CFMutableDataRef, CFDataGetTypeID);

impl CFMutableData {
  pub fn new(capacity: usize) -> Result<CFMutableData, ()> {
    unsafe {
      let result = CFDataCreateMutable(kCFAllocatorDefault, capacity.to_CFIndex());
      if result != ptr::null() {
        Ok(TCFType::wrap_under_create_rule(result))
      } else {
        Err(())
      }
    }
  }

  #[inline]
  pub fn len(&self) -> CFIndex {
    unsafe { CFDataGetLength(mem::transmute(self.as_concrete_TypeRef())) }
  }

  #[inline]
  pub fn bytes<'a>(&'a self) -> &'a [u8] {
    unsafe {
      slice::from_raw_parts(CFDataGetBytePtr(mem::transmute(self.0)),
                            self.len() as usize)
    }
  }
}


#[link(name = "CoreFoundation", kind = "framework")]
extern "C" {
  fn CFDataCreateMutable(allocator: CFAllocatorRef, capacity: CFIndex) -> CFMutableDataRef;
}
