use super::voxel::VoxelBuffer;
use crate::vec3::*;
use crate::aabb::*;

pub struct DenseBuffer {
	size_x: i32,
	size_y: i32,
	size_z: i32,
	data: Vec<Vec3>,
	ws_bounds: AABB
}

impl DenseBuffer {
	pub fn new(size3d: (i32, i32, i32), ws_bounds: AABB) -> DenseBuffer {
		let size = (size3d.0 * size3d.1 * size3d.2) as usize;
		let mut data = Vec::<Vec3>::new();
		data.resize(size, Vec3::zero());

		DenseBuffer {
			size_x: size3d.0,
			size_y: size3d.1,
			size_z: size3d.2,
			data: data,
			ws_bounds: ws_bounds
		}
	}

	fn sample_by_voxel_position(&self, p: Vec3) -> Vec3 {
		let vp = p + vec3(0.5, 0.5, 0.5);
		let (x, y, z) = (vp.x as i32, vp.y as i32, vp.z as i32);

		if x < 0 || y < 0 || z < 0 || x >= self.size_x || y >= self.size_y || z >= self.size_z {
			Vec3::zero()
		} else {
			self.read(x, y, z)
		}
	}

	fn index(&self, i: i32, j: i32, k: i32) -> usize {
		(i + j * self.size_x + k * self.size_x * self.size_y) as usize
	}
}

impl VoxelBuffer for DenseBuffer {
	fn sample_by_local_position(&self, u: f32, v: f32, w: f32) -> Vec3 {
		if u < 0.0 || v < 0.0 || w < 0.0 || u >= 1.0 || v >= 1.0 || w >= 1.0 {
			Vec3::zero()
		} else {
			let i = (0.5 + u * (self.size_x as f32)) as i32;
			let j = (0.5 + v * (self.size_y as f32)) as i32;
			let k = (0.5 + w * (self.size_z as f32)) as i32;
			let ix = self.index(i, j, k);

			if ix >= self.data.len() {
				Vec3::zero()
			} else {
				self.data[ix]
			}
		}
	}

	// nearest point
	//fn sample_by_world_position(&self, p: Vec3) -> Vec3 {
	//	let lp = self.world_to_voxel(p) / self.get_sizef();
	//	self.sample_by_local_position(lp.x, lp.y, lp.z)
	//}

	// linear interpolation
	fn sample_by_world_position(&self, p: Vec3) -> Vec3 {
		let vp = self.world_to_voxel(p);
		let f = (vp - vec3(0.5, 0.5, 0.5)).floor();
		let a = vp - vec3(0.5, 0.5, 0.5) - f;

		let v000 = self.sample_by_voxel_position(f);
		let v001 = self.sample_by_voxel_position(f + vec3(0.0, 0.0, 1.0));
		let v010 = self.sample_by_voxel_position(f + vec3(0.0, 1.0, 0.0));
		let v011 = self.sample_by_voxel_position(f + vec3(0.0, 1.0, 1.0));
		let v100 = self.sample_by_voxel_position(f + vec3(1.0, 0.0, 0.0));
		let v101 = self.sample_by_voxel_position(f + vec3(1.0, 0.0, 1.0));
		let v110 = self.sample_by_voxel_position(f + vec3(1.0, 1.0, 0.0));
		let v111 = self.sample_by_voxel_position(f + vec3(1.0, 1.0, 1.0));

		let front = lerp(lerp(v000, v100, a.x), lerp(v010, v110, a.x), a.y);
		let back = lerp(lerp(v001, v101, a.x), lerp(v011, v111, a.x), a.y);

		lerp(front, back, a.z)
	}

	fn world_to_voxel(&self, p: Vec3) -> Vec3 {
		fit(p, self.ws_bounds.min, self.ws_bounds.max, Vec3::zero(), self.get_sizef())
	}
	fn voxel_to_world(&self, p: Vec3) -> Vec3 {
		fit(p, Vec3::zero(), self.get_sizef(), self.ws_bounds.min, self.ws_bounds.max)
	}

	fn get_size(&self) -> (i32, i32, i32) {
		(self.size_x, self.size_y, self.size_z)
	}
	fn get_sizef(&self) -> Vec3 {
		vec3(self.size_x as f32, self.size_y as f32, self.size_z as f32)
	}
	fn get_ws_bounds(&self) -> AABB {
		self.ws_bounds
	}

	fn get_occupancy(&self) -> f32 { 1.0 }

	// Raw read & write
	fn read(&self, i: i32, j: i32, k: i32) -> Vec3 {
		self.data[self.index(i, j, k)]
	}
	fn write(&mut self, i: i32, j: i32, k: i32, value: Vec3) -> () {
		let ix = self.index(i, j, k);
		self.data[ix] = value;
	}
}
