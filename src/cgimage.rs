#![allow(improper_ctypes, non_camel_case_types, non_upper_case_globals)]

use cfmutabledata::CFMutableData;
use cgimagedestination::CGImageDestination;
use core_foundation::base::{CFRelease, CFTypeID, TCFType};
use core_graphics::color_space::{CGColorSpace, CGColorSpaceRef};
use core_graphics::context::{CGContext, CGContextRef};
use core_graphics::data_provider::{CGDataProvider, CGDataProviderRef};
use core_graphics::geometry::{CGPoint, CGRect, CGSize};
use libc::{c_void, size_t};
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::ptr;

pub type bool = u8;

pub type CGColorRenderingIntent = i32;
pub const kCGRenderingIntentDefault: CGColorRenderingIntent = 0;
// TODO: make this a lib and uncomment these.
// pub const kCGRenderingIntentAbsoluteColorimetric: CGColorRenderingIntent = 1;
// pub const kCGRenderingIntentRelativeColorimetric: CGColorRenderingIntent = 2;
// pub const kCGRenderingIntentPerceptual: CGColorRenderingIntent = 3;
// pub const kCGRenderingIntentSaturation: CGColorRenderingIntent = 4;

pub type CGImageAlphaInfo = u32;
pub const kCGImageAlphaNoneSkipFirst: CGImageAlphaInfo = 6;

types_CFType!(CGImage, CGImageRef, __CGImage);
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

  pub fn jpeg_data(&self) -> Result<Vec<u8>, ()> {
    let data = try!(CFMutableData::new(0));
    let dest = CGImageDestination::jpg_destination_with_data(&data);
    dest.add_image(self);
    try!(dest.finalize());
    let mut vec = Vec::new();
    vec.extend_from_slice(data.bytes());
    Ok(vec)
  }

  pub fn write_jpeg<T>(&self, mut w: T) -> Result<(), ()>
    where T: Write + Sized {
    let bytes = try!(self.jpeg_data());
    try!(w.write_all(&bytes).map_err(|_| ()));
    Ok(())
  }

  pub fn save_jpeg_to_file(&self, path: &Path) -> Result<(), ()> {
    let out_file = try!(File::create(path).map_err(|_| ()));
    try!(self.write_jpeg(out_file));
    Ok(())
  }
}

pub fn create_device_gray_color_space() -> CGColorSpace {
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
// Functionally identical to CFRelease, but with safety guarantees that I don't need.
// fn CGImageRelease(image: CFTypeRef);
}
