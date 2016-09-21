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
use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::vec::Vec;

fn save_image_to_file(image: &CGImage, path: &Path) -> Result<(), ()> {
  let data = try!(CFMutableData::new(0));
  let dest = CGImageDestination::jpg_destination_with_data(&data);
  dest.add_image(image);
  try!(dest.finalize());

  let mut out_file = File::create(path).unwrap();
  out_file.write(data.bytes());

  Ok(())
}

fn main() {
  let pathbuf = env::current_dir().unwrap();
  let filename = pathbuf
			//.parent().unwrap()
			.join("sample").join("biel.jpg");
  let mut f = File::open(filename).unwrap();
  let mut image_buffer: Vec<u8> = Vec::new();
  f.read_to_end(&mut image_buffer).unwrap();

  let img = CGImage::new(&image_buffer).unwrap();

  let grayscale_space = cgimage::create_gray_color_space();
  let context = CGContext::create_bitmap_context(img.width(),
                                                 img.height(),
                                                 8,
                                                 img.bytes_per_row(),
                                                 &grayscale_space,
                                                 0);
  img.draw_into_context(&context);

  let bw_image = CGImage::image_from_bitmap_context(&context).unwrap();
  save_image_to_file(&bw_image, Path::new("foobar.jpg")).unwrap();
  println!("Wrote file.");
}
