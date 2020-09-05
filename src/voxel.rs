
pub struct VoxelBuffer {
	size_x: i32,
	size_y: i32,
	size_z: i32,
	data: Vec<f32>,
}

impl VoxelBuffer {
	pub fn new(size_x: i32, size_y: i32, size_z: i32) -> VoxelBuffer {
		let size = (size_x * size_y * size_z) as usize;
		let mut data = Vec::<f32>::new();
		data.resize(size, 0.0);

		VoxelBuffer { size_x: size_x, size_y: size_y, size_z: size_z, data: data }
	}

	pub fn read(&self, i: i32, j: i32, k: i32) -> f32 {
		self.data[self.index(i, j, k) as usize]
	}

	pub fn write(&mut self, i: i32, j: i32, k: i32, value: f32) -> () {
		let ix = self.index(i, j, k) as usize;
		self.data[ix] = value;
	}

	fn index(&self, i: i32, j: i32, k: i32) -> i32 {
		i + j * self.size_x + k * self.size_x * self.size_y
	}
}
