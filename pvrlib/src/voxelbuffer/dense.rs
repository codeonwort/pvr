use super::VoxelBuffer;
use crate::math::vec3::*;
use crate::math::ray::Ray;
use crate::math::aabb::AABB;

pub struct DenseBuffer {
	size_x: i32,
	size_y: i32,
	size_z: i32,
	data: Vec<Vec3>,
}

impl DenseBuffer {
	pub fn new(size3d: (i32, i32, i32)) -> DenseBuffer {
		let size = (size3d.0 * size3d.1 * size3d.2) as usize;
		let mut data = Vec::<Vec3>::new();
		data.resize(size, Vec3::zero());

		DenseBuffer {
			size_x: size3d.0,
			size_y: size3d.1,
			size_z: size3d.2,
			data: data
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
			let fx = 0.5 + u * (self.size_x as f32);
			let fy = 0.5 + v * (self.size_y as f32);
			let fz = 0.5 + w * (self.size_z as f32);
			let f = vec3(fx, fy, fz);
			let a = f - f.floor();

			let read_raw = |vf: Vec3| -> Vec3 {
				let ix = self.index(vf.x as i32, vf.y as i32, vf.z as i32);
				if ix >= self.data.len() {
					Vec3::zero()
				} else {
					self.data[ix]
				}
			};

			let v000 = read_raw(f);
			let v001 = read_raw(f + vec3(0.0, 0.0, 1.0));
			let v010 = read_raw(f + vec3(0.0, 1.0, 0.0));
			let v011 = read_raw(f + vec3(0.0, 1.0, 1.0));
			let v100 = read_raw(f + vec3(1.0, 0.0, 0.0));
			let v101 = read_raw(f + vec3(1.0, 0.0, 1.0));
			let v110 = read_raw(f + vec3(1.0, 1.0, 0.0));
			let v111 = read_raw(f + vec3(1.0, 1.0, 1.0));

			let front = lerp(lerp(v000, v100, a.x), lerp(v010, v110, a.x), a.y);
			let back = lerp(lerp(v001, v101, a.x), lerp(v011, v111, a.x), a.y);
			let final_value = lerp(front, back, a.z);

			final_value
		}
	}

	fn get_size(&self) -> (i32, i32, i32) {
		(self.size_x, self.size_y, self.size_z)
	}
	fn get_sizef(&self) -> Vec3 {
		vec3(self.size_x as f32, self.size_y as f32, self.size_z as f32)
	}

	// #todo-emptyspace: Any way to skip empty spaces for dense buffer?
	// Keep internal sparse buffer only to find intersections?
	fn find_intersections(&self, ray: Ray, world_bounds: AABB) -> Vec<(f32, f32)> {
		let mut intervals = Vec::new();
        if let Some(v) = world_bounds.intersect(ray) {
            intervals.push(v);
        }

		// #todo-emptyspace: Negative numbers and NaN ???
		//if intervals.len() > 0 {
		//	println!("=== dense intervals ===");
		//	for (t0,t1) in &intervals {
		//		println!("{}, {}", t0, t1);
		//	}
		//}

        intervals
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
