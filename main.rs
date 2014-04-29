/* main.rs */

/* Parsing info from http://dds.cr.usgs.gov/srtm/version2_1/SRTM3/ */

use std::io::File;
use std::vec::Vec;

fn main() {

	let p = Path::new("example.hgt");

	let mut file = match File::open(&p) {
    	Ok(f) => f,
    	Err(e) => fail!("file error: {}", e)
	};

	let bytes = match file.read_to_end() {
		Ok(b) => b,
    	Err(e) => fail!("read error: {}", e)
	};

	println!("Done! {:?}", bytes);

	let side = ((bytes.len() / 2) as f64).sqrt();

	if side != side.round() {
		fail!("Not square!");
	}

	let size = side as uint;

	// Break this out, otherwise we get bytes lifetime issue...
	let f = |z| bytes.get(z);

	let heights: Vec<Vec<&u8>> = Vec::from_fn(size, |idy| { 
		Vec::from_fn(size, |idx| f(idy * size + idx * 2))
	});

	println!("Done! {:?}", heights);

	println!("Value: {}", heights.get(1).get(1));

}