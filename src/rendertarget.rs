use crate::vec3::*;

pub struct RenderTarget {
	pixels: Vec<Vec3>,
	width: usize,
	height: usize
}

impl RenderTarget {

	pub fn new(width: usize, height: usize) -> RenderTarget {
		let mut pixels: Vec<Vec3> = Vec::new();
		let black = Vec3::zero();
		pixels.resize(width * height, black);
		RenderTarget { pixels: pixels, width: width, height: height }
	}

	// #todo: Redundancy with generate_ldr_buffer()
	pub fn copy_to(&self, buffer: &mut Vec<u8>) {
		let buffer_size = (self.width * self.height * 3) as usize;
		buffer.resize(buffer_size, 0);

		let mut ptr = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				let px: Vec3 = self.get(x as i32, y as i32);
				let r: u8 = (((px.x * 255.0) as u32) & 0xff) as u8;
				let g: u8 = (((px.y * 255.0) as u32) & 0xff) as u8;
				let b: u8 = (((px.z * 255.0) as u32) & 0xff) as u8;
				buffer[ptr] = r;
				buffer[ptr+1] = g;
				buffer[ptr+2] = b;
				ptr += 3;
			}
		}
	}

	pub fn generate_ldr_buffer(&self) -> Vec<u8> {
		let mut buffer: Vec<u8> = Vec::new();
		let buffer_size = (self.width * self.height * 3) as usize;
		buffer.resize(buffer_size, 0);

		let mut ptr = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				let px: Vec3 = self.get(x as i32, y as i32);
				let r: u8 = (((px.x * 255.0) as u32) & 0xff) as u8;
				let g: u8 = (((px.y * 255.0) as u32) & 0xff) as u8;
				let b: u8 = (((px.z * 255.0) as u32) & 0xff) as u8;
				buffer[ptr] = r;
				buffer[ptr+1] = g;
				buffer[ptr+2] = b;
				ptr += 3;
			}
		}

		buffer
	}

	pub fn set(&mut self, x: i32, y:i32, pixel: Vec3) {
		let ix = self.index(x, y) as usize;
		self.pixels[ix] = pixel
	}

	// Returns black color for out of range
	pub fn get(&self, x: i32, y:i32) -> Vec3 {
		if self.contains(x, y) {
			self.pixels[self.index(x, y) as usize]
		} else {
			Vec3::zero()
		}
	}

	pub fn get_width(&self) -> usize { self.width }
	pub fn get_height(&self) -> usize { self.height }

	pub fn contains(&self, x: i32, y:i32) -> bool {
		0 <= x && x < (self.width as i32) && 0 <= y && y < (self.height as i32)
	}

	fn index(&self, x: i32, y: i32) -> i32 {
		y * (self.width as i32) + x
	}

}
