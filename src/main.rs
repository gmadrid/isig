mod cgimage;

use cgimage::CGImage;
use std::env;
use std::vec::Vec;
use std::fs::File;
use std::io::Read;

fn main() {
	let pathbuf = env::current_dir().unwrap();
	let filename = pathbuf.parent().unwrap().join("sample").join("biel.jpg");
	let mut f = File::open(filename).unwrap();
	let mut image_buffer : Vec<u8> = Vec::new();
	f.read_to_end(&mut image_buffer).unwrap();

	CGImage::new(&image_buffer);
}
