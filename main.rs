/* main.rs */

/* Parsing info from http://dds.cr.usgs.gov/srtm/version2_1/SRTM3/ */

use std::io::{File, IoError};
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

	let heights: Vec<Vec<u8>> = Vec::from_fn(size, |idy| { 
		Vec::from_fn(size, |idx| *bytes.get(idy * size + idx * 2))
	});

	println!("Done! {:?}", heights);

	println!("Value: {}", heights.get(1).get(1));

	let val = match load(&p) {
		Ok(v) => v,
		Err(e) => fail!("Load failed: {}", e)
	};

	println!("Value: {}", val.get(1).get(1));
}

fn load(path: &Path) -> Result<Vec<Vec<i16>>, IoError> { 
	
	let filesize: u64 = match path.stat() {
		Ok(stat) => stat.size,
		Err(e) => fail!("Couldn't read file <{}> size: {}", path.		filename(), e)
	};

	let sidef = ((filesize / 2) as f64).sqrt();

	if sidef * sidef * 2.0 != (filesize as f64) {
		fail!("Not square");
	}

	let side = sidef as uint;

	let mut file = try!(File::open(path));

	let mut result: Vec<Vec<i16>> = Vec::from_fn(side, |idy| { 
		Vec::with_capacity(side)
	});
	
	let mut counter = 0;

	while !file.eof() {
		let data = match file.read_be_i16() {
			Ok(d) => d,
			Err(e) => return Ok(result)
		};

		let y = counter / side;
		let x = counter % side;

		let a = result.get_mut(y);

		a.insert(x,data);

		counter += 1;
	}

	return Ok(result);
}