#[macro_use]
extern crate core_foundation;
extern crate core_foundation_sys;
extern crate core_graphics;
extern crate libc;

mod cgimage;
mod cfmutabledata;
mod cgimagedestination;

use cfmutabledata::CFMutableData;
use cgimage::CGImage;
use cgimagedestination::CGImageDestination;
use core_graphics::context::CGContext;
use core_graphics::color_space::CGColorSpace;
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::vec::Vec;

fn save_image_to_file(image: &CGImage, path: &Path) -> Result<(), ()> {
  let out_file = try!(File::create(path).map_err(|_| ()));
  try!(write_jpeg(image, out_file));
  Ok(())
}

fn write_jpeg<T>(image: &CGImage, mut w: T) -> Result<(), ()>  where T: Write + Sized {
  // let data = try!(CFMutableData::new(0));
  // let dest = CGImageDestination::jpg_destination_with_data(&data);
  // dest.add_image(image);
  // try!(dest.finalize());
  let bytes = try!(jpeg_data_for_image(image));
  try!(w.write_all(&bytes).map_err(|_| ()));
  Ok(())
}

fn jpeg_data_for_image(image: &CGImage) -> Result<Vec<u8>, ()> {
  let data = try!(CFMutableData::new(0));
  let dest = CGImageDestination::jpg_destination_with_data(&data);
  dest.add_image(image);
  try!(dest.finalize());
  let mut vec = Vec::new();
  vec.extend_from_slice(data.bytes());
  Ok(vec)
//  Ok(Box::<[u8]>::new(data.bytes()));
//  try!(w.write_all(data.bytes()).map_err(|_| ()));
}

fn main() {
  let pathbuf = env::current_dir().unwrap();
  let filename = pathbuf
			.parent().unwrap()
			.join("sample").join("biel.jpg");
  let mut f = File::open(filename).unwrap();
  let mut image_buffer: Vec<u8> = Vec::new();
  f.read_to_end(&mut image_buffer).unwrap();

  let img = CGImage::new(&image_buffer).unwrap();

  let grayscale_space = cgimage::create_gray_color_space();
  let context = CGContext::create_bitmap_context(img.width(),
                                                 img.height(),
                                                 8,
                                                 0,
                                                 &grayscale_space,
                                                 0);
  img.draw_into_context(&context);

  let bw_image = CGImage::image_from_bitmap_context(&context).unwrap();
  save_image_to_file(&bw_image, Path::new("foobar.jpg")).unwrap();

  let small_context = CGContext::create_bitmap_context(8, 8, 8, 0, &grayscale_space, 0);
  bw_image.draw_into_context(&small_context);
  let small_image = CGImage::image_from_bitmap_context(&small_context).unwrap();
  save_image_to_file(&small_image, Path::new("smallfoo.jpg")).unwrap();

  let smcolor_context = CGContext::create_bitmap_context(8, 8, 8, 0, &CGColorSpace::create_device_rgb(), 6);
  img.draw_into_context(&smcolor_context);
  let smcolor_image = CGImage::image_from_bitmap_context(&smcolor_context).unwrap();
  save_image_to_file(&smcolor_image, Path::new("smallcolor.jpg")).unwrap();
  println!("Wrote file.");
}
