/* main.rs */

/* Parsing info from http://dds.cr.usgs.gov/srtm/version2_1/SRTM3/ */

use std::io::File;

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

	println!("Done! {}", bytes);
}