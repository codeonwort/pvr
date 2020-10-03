use crate::vec3::*;
use crate::aabb::*;

pub struct VoxelBuffer {
	size_x: i32,
	size_y: i32,
	size_z: i32,
	data: Vec<Vec3>,
	ws_bounds: AABB
}

impl VoxelBuffer {
	pub fn new(size3d: (i32, i32, i32), ws_bounds: AABB) -> VoxelBuffer {
		let size = (size3d.0 * size3d.1 * size3d.2) as usize;
		let mut data = Vec::<Vec3>::new();
		data.resize(size, Vec3::zero());

		VoxelBuffer {
			size_x: size3d.0,
			size_y: size3d.1,
			size_z: size3d.2,
			data: data,
			ws_bounds: ws_bounds
		}
	}

	pub fn sample(&self, u: f32, v: f32, w: f32) -> Vec3 {
		if u < 0.0 || v < 0.0 || w < 0.0 || u >= 1.0 || v >= 1.0 || w >= 1.0 {
			Vec3::zero()
		} else {
			let i = (u * (self.size_x as f32)) as i32;
			let j = (v * (self.size_y as f32)) as i32;
			let k = (w * (self.size_z as f32)) as i32;
			let ix = self.index(i, j, k);

			if ix >= self.data.len() {
				Vec3::zero()
			} else {
				self.data[ix]
			}
		}
	}

	pub fn sample_by_world_position(&self, p: Vec3) -> Vec3 {
		let lp = self.world_to_voxel(p) / self.get_sizef();
		self.sample(lp.x, lp.y, lp.z)
	}

	pub fn world_to_voxel(&self, p: Vec3) -> Vec3 {
		fit(p, self.ws_bounds.min, self.ws_bounds.max, Vec3::zero(), self.get_sizef())
	}
	pub fn voxel_to_world(&self, p: Vec3) -> Vec3 {
		fit(p, Vec3::zero(), self.get_sizef(), self.ws_bounds.min, self.ws_bounds.max)
	}

	pub fn get_size(&self) -> (i32, i32, i32) {
		(self.size_x, self.size_y, self.size_z)
	}
	pub fn get_sizef(&self) -> Vec3 {
		vec3(self.size_x as f32, self.size_y as f32, self.size_z as f32)
	}
	pub fn get_ws_bounds(&self) -> AABB {
		self.ws_bounds
	}

	// Raw read & write
	pub fn read(&self, i: i32, j: i32, k: i32) -> Vec3 {
		self.data[self.index(i, j, k)]
	}
	pub fn write(&mut self, i: i32, j: i32, k: i32, value: Vec3) -> () {
		let ix = self.index(i, j, k);
		self.data[ix] = value;
	}
	fn index(&self, i: i32, j: i32, k: i32) -> usize {
		(i + j * self.size_x + k * self.size_x * self.size_y) as usize
	}
}
