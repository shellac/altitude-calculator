extern crate exportpng;

use std::io::File;

mod vec2d;

fn main() {

	let p = Path::new("example.hgt");

	//let mut file = try!(File::open(&p));

	let mut file = match File::open(&p) {
    	Ok(f) => f,
    	Err(e) => fail!("file error: {}", e)
	};

	let bytes = match file.read_to_end() {
		Ok(b) => b,
    	Err(e) => fail!("read error: {}", e)
	};

	let sidef = ((bytes.len() / 2) as f64).sqrt();

	if sidef * sidef * 2.0 != bytes.len() as f64 {
		fail!("Not square!");
	}

	let side = sidef as uint;

	// Get each pair of bytes and make an i16
	let newBytes: Vec<i16> = bytes.as_slice()
		.chunks(2)
		.map(|a| toSignedShort(a[0], a[1]))
		.collect();

	let heightMap = vec2d::Vec2D::make(side, side, ~newBytes);

	let image: Vec<u8> = heightMap.content
		.iter()
		.flat_map(|&el| toPixel(el))
		.collect();

	for i in image.iter() {
		println!("{}", i);
	}

	match exportpng::to_file(
		"test.png",
		heightMap.width as u32,
		heightMap.height as u32,
		image.as_slice()
		) {
        Ok(_) => println!("Saved!"),
        Err(reason) => println!("Could not save because: {}", reason),
    }
}

// For flat mapping from signed ints to iterator over pixel vals
fn toPixel(val: i16) -> std::vec::MoveItems<u8> {
	let pix = vec!(val as u8, val as u8, val as u8, 255u8);
	pix.move_iter()
}

fn toSignedShort(msb: u8, lsb: u8) -> i16 {
	msb as i16 << 8 | lsb as i16
}

#[test]
fn checkConversions() {
	assert_eq!(toSignedShort(0xff, 0xff), -1);
	assert_eq!(toSignedShort(0, 0xff), 255);
	assert_eq!(toSignedShort(1, 0xff), 511);
	assert_eq!(toSignedShort(0xff, 0xfe), -2);
}
