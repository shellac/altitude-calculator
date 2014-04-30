use std::vec::Vec;

pub struct Vec2D<T> {
	width: uint,
	height: uint,
	content: ~Vec<T>
}

impl<T> Vec2D<T> {

	pub fn new(width: uint, height: uint) -> Vec2D<T> {
		Vec2D { 
			width: width, 
			height: height, 
			content: ~Vec::with_capacity(width * height) 
		}
	}

	pub fn make(width: uint, height: uint, content: ~Vec<T>) -> Vec2D<T> {
		Vec2D { 
			width: width, 
			height: height, 
			content: content 
		}
	}

	fn twoToOne(&self, x: uint, y: uint) -> uint {
		y * self.width + x
	}

	// Compiler told me to do this :-)
	pub fn get<'a>(&'a self, x: uint, y: uint) -> &'a T {
		self.content.get(self.twoToOne(x, y))
	}
}
