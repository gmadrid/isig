#![allow(improper_ctypes, non_camel_case_types, non_upper_case_globals)]

use core_foundation::base::{CFTypeID, CFTypeRef, TCFType};
use core_graphics::color_space::{CGColorSpace, CGColorSpaceRef};
use core_graphics::context::{CGContext, CGContextRef};
use core_graphics::data_provider::{CGDataProvider, CGDataProviderRef};
use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use libc::{c_void, size_t};
use std::ptr;

pub type bool = u8;

pub type CGColorRenderingIntent = i32;
pub const kCGRenderingIntentDefault: CGColorRenderingIntent = 0;
// TODO: make this a lib and uncomment these.
// pub const kCGRenderingIntentAbsoluteColorimetric: CGColorRenderingIntent = 1;
// pub const kCGRenderingIntentRelativeColorimetric: CGColorRenderingIntent = 2;
// pub const kCGRenderingIntentPerceptual: CGColorRenderingIntent = 3;
// pub const kCGRenderingIntentSaturation: CGColorRenderingIntent = 4;

#[repr(C)]
pub struct __CGImage;

pub type CGImageRef = *const __CGImage;

pub struct CGImage(CGImageRef);

impl Drop for CGImage {
  fn drop(&mut self) {
    unsafe { CGImageRelease(self.as_CFTypeRef()) }
  }
}

impl_TCFType!(CGImage, CGImageRef, CGImageGetTypeID);

impl CGImage {
  pub fn new(bytes: &[u8]) -> Result<CGImage, ()> {
    unsafe {
      let data_provider = CGDataProvider::from_buffer(bytes);
      // TODO: Is this the right value for shouldInterpolate?
      let result = CGImageCreateWithJPEGDataProvider(data_provider.as_concrete_TypeRef(),
                                                     ptr::null(),
                                                     0,
                                                     kCGRenderingIntentDefault);
      if result != ptr::null() {
        Ok(TCFType::wrap_under_create_rule(result))
      } else {
        Err(())
      }
    }
  }

  pub fn image_from_bitmap_context(context: &CGContext) -> Result<CGImage, ()> {
    unsafe {
      let result = CGBitmapContextCreateImage(context.as_concrete_TypeRef());
      if result != ptr::null() {
        Ok(TCFType::wrap_under_create_rule(result))
      } else {
        println!("FAUILD HERE");
        Err(())
      }
    }
  }

  // pub fn bytes_per_row(&self) -> size_t {
  //   unsafe { CGImageGetBytesPerRow(self.as_concrete_TypeRef()) }
  // }

  pub fn height(&self) -> size_t {
    unsafe { CGImageGetHeight(self.as_concrete_TypeRef()) }
  }

  pub fn width(&self) -> size_t {
    unsafe { CGImageGetWidth(self.as_concrete_TypeRef()) }
  }

  pub fn draw_into_context(&self, context: &CGContext) {
    // TODO: This is just here until context.draw_image() is implemented in core_graphics.
    unsafe {
      let height = CGBitmapContextGetHeight(context.as_concrete_TypeRef());
      let width = CGBitmapContextGetWidth(context.as_concrete_TypeRef());
      CGContextDrawImage(context.as_concrete_TypeRef(),
                         CGRect::new(&CGPoint::new(0.0, 0.0),
                                     &CGSize::new(width as f64, height as f64)),
                         self.as_concrete_TypeRef())
    }
  }
}

pub fn create_gray_color_space() -> CGColorSpace {
  unsafe {
    let color_space = CGColorSpaceCreateDeviceGray();
    TCFType::wrap_under_create_rule(color_space)
  }
}

#[link(name = "CoreGraphics", kind = "framework")]
extern "C" {
  fn CGBitmapContextCreateImage(context: CGContextRef) -> CGImageRef;
  fn CGBitmapContextGetHeight(context: CGContextRef) -> size_t;
  fn CGBitmapContextGetWidth(context: CGContextRef) -> size_t;
  fn CGColorSpaceCreateDeviceGray() -> CGColorSpaceRef;
  fn CGContextDrawImage(context: CGContextRef, rect: CGRect, image: CGImageRef);

  fn CGImageCreateWithJPEGDataProvider(dataProvider: CGDataProviderRef,
                                       decode: *const c_void,
                                       shouldInterpolate: bool,
                                       intent: CGColorRenderingIntent)
      -> CGImageRef;
//  fn CGImageGetBytesPerRow(image: CGImageRef) -> size_t;
  fn CGImageGetHeight(image: CGImageRef) -> size_t;
  fn CGImageGetTypeID() -> CFTypeID;
  fn CGImageGetWidth(image: CGImageRef) -> size_t;
  fn CGImageRelease(image: CFTypeRef);
}
