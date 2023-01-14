use super::VoxelBuffer;
use crate::math::vec3::*;
use crate::math::ray::Ray;
use crate::math::aabb::AABB;

pub struct DenseField<T> {
	size_x: i32,
	size_y: i32,
	size_z: i32,
	data: Vec<T>,
}

impl<T: Clone> DenseField<T> {
	pub fn new(size: (i32, i32, i32), value: T) -> DenseField<T> {
		let total = (size.0 * size.1 * size.2) as usize;
		let mut data = Vec::<T>::new();
		data.resize(total, value);

		DenseField {
			size_x: size.0,
			size_y: size.1,
			size_z: size.2,
			data: data
		}
	}
	
	fn index(&self, i: i32, j: i32, k: i32) -> usize {
		(i + j * self.size_x + k * self.size_x * self.size_y) as usize
	}
}

impl VoxelBuffer for DenseField<vec3f> {
	fn sample_by_local_position(&self, u: f32, v: f32, w: f32) -> vec3f {
		if u < 0.0 || v < 0.0 || w < 0.0 || u >= 1.0 || v >= 1.0 || w >= 1.0 {
			vec3f::zero()
		} else {
			let fx = 0.5 + u * (self.size_x as f32);
			let fy = 0.5 + v * (self.size_y as f32);
			let fz = 0.5 + w * (self.size_z as f32);
			let f = vec3(fx, fy, fz);
			let a = f - f.floor();

			let read_raw = |vf: vec3f| -> vec3f {
				let ix = self.index(vf.x as i32, vf.y as i32, vf.z as i32);
				if ix >= self.data.len() {
					vec3f::zero()
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
	fn get_sizef(&self) -> vec3f {
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
	fn read(&self, i: i32, j: i32, k: i32) -> vec3f {
		self.data[self.index(i, j, k)]
	}
	fn write(&mut self, i: i32, j: i32, k: i32, value: vec3f) -> () {
		let ix = self.index(i, j, k);
		self.data[ix] = value;
	}
}
