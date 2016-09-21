#[macro_use]
extern crate core_foundation;
extern crate core_foundation_sys;
extern crate core_graphics;
extern crate libc;

#[macro_use]
mod macros;
mod cgimage;
mod cfmutabledata;
mod cgimagedestination;

use cgimage::CGImage;
use core_graphics::color_space::CGColorSpace;
use core_graphics::context::CGContext;
use std::env;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;

fn main() {
  let pathbuf = env::current_dir().unwrap();
  let filename = pathbuf.parent()
    .unwrap()
    .join("sample")
    .join("biel.jpg");
  let mut f = File::open(filename).unwrap();
  let mut image_buffer: Vec<u8> = Vec::new();
  f.read_to_end(&mut image_buffer).unwrap();

  let img = CGImage::new(&image_buffer).unwrap();

  let grayscale_space = cgimage::create_device_gray_color_space();
  let context =
    CGContext::create_bitmap_context(img.width(), img.height(), 8, 0, &grayscale_space, 0);
  img.draw_into_context(&context);

  let bw_image = CGImage::image_from_bitmap_context(&context).unwrap();
  bw_image.save_jpeg_to_file(Path::new("foobar.jpg")).unwrap();

  let small_context = CGContext::create_bitmap_context(8, 8, 8, 0, &grayscale_space, 0);
  bw_image.draw_into_context(&small_context);
  let small_image = CGImage::image_from_bitmap_context(&small_context).unwrap();
  small_image.save_jpeg_to_file(Path::new("smallfoo.jpg")).unwrap();

  let smcolor_context = CGContext::create_bitmap_context(8,
                                                         8,
                                                         8,
                                                         0,
                                                         &CGColorSpace::create_device_rgb(),
                                                         cgimage::kCGImageAlphaNoneSkipFirst);
  img.draw_into_context(&smcolor_context);
  let smcolor_image = CGImage::image_from_bitmap_context(&smcolor_context).unwrap();
  smcolor_image.save_jpeg_to_file(Path::new("smallcolor.jpg")).unwrap();

  let big_context = CGContext::create_bitmap_context(200, 200, 8, 0, &grayscale_space, 0);
  small_image.draw_into_context(&big_context);
  let big_image = CGImage::image_from_bitmap_context(&big_context).unwrap();
  big_image.save_jpeg_to_file(Path::new("embiggened.jpg")).unwrap();

  println!("Wrote file.");
}
