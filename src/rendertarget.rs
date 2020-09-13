// To layout struct in a way compatible with C/C++,
// use #[repr(C)] attributes. (Chapter 21)

#[derive(Copy, Clone, Default)]
pub struct Pixel {
	pub r: f32,
	pub g: f32,
	pub b: f32
}

pub struct RenderTarget {
	pixels: Vec<Pixel>,
	width: usize,
	height: usize
}

impl RenderTarget {

	pub fn new(width: usize, height: usize) -> RenderTarget {
		let mut pixels: Vec<Pixel> = Vec::new();
		let black = Pixel { r: 0.0, g: 0.0, b: 0.0 };
		pixels.resize(width * height, black);
		RenderTarget { pixels: pixels, width: width, height: height }
	}

	pub fn generate_ldr_buffer(&self) -> Vec<u8> {
		let mut buffer: Vec<u8> = Vec::new();
		let buffer_size = (self.width * self.height * 3) as usize;
		buffer.resize(buffer_size, 0);

		let mut ptr = 0;
		for y in 0..self.height {
			for x in 0..self.width {
				let px: Pixel = self.get(x as i32, y as i32);
				let r: u8 = (((px.r * 255.0) as u32) & 0xff) as u8;
				let g: u8 = (((px.g * 255.0) as u32) & 0xff) as u8;
				let b: u8 = (((px.b * 255.0) as u32) & 0xff) as u8;
				buffer[ptr] = r;
				buffer[ptr+1] = g;
				buffer[ptr+2] = b;
				ptr += 3;
			}
		}

		buffer
	}

	pub fn set(&mut self, x: i32, y:i32, pixel: Pixel) {
		let ix = self.index(x, y) as usize;
		self.pixels[ix] = pixel
	}

	// Returns black color for out of range
	pub fn get(&self, x: i32, y:i32) -> Pixel {
		if self.contains(x, y) {
			self.pixels[self.index(x, y) as usize]
		} else {
			Pixel { r: 0.0, g: 0.0, b: 0.0 }
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
