use crate::math::vec3::*;
use crate::primitive::*;
use crate::voxelbuffer::VoxelBuffer;

pub struct Line {
	pub	p0: Vec3,
	pub	p1: Vec3,
	pub	radius: f32
}

impl Line {
	pub fn density(&self, p: Vec3) -> Vec3 {
		let dir = (self.p1 - self.p0).normalize();
		let dist = dir.cross(p - self.p0).length();

		if dist <= self.radius {
			Vec3::one()
		} else {
			Vec3::zero()
		}
	}
}

impl Primitive for Line {}

impl RasterizationPrimitive for Line {
	fn rasterize(&self, voxel_buffer: &mut dyn VoxelBuffer) {
		let p0 = voxel_buffer.world_to_voxel(self.p0);
		let p1 = voxel_buffer.world_to_voxel(self.p1);
		let aug = vec3(self.radius, self.radius, self.radius);

		let p_min = vec3(f32::min(p0.x, p1.x), f32::min(p0.y, p1.y), f32::min(p0.z, p1.z)) - aug;
		let p_max = vec3(f32::max(p0.x, p1.x), f32::max(p0.y, p1.y), f32::max(p0.z, p1.z)) + aug;
        let (x_min, y_min, z_min) = (p_min.x as i32, p_min.y as i32, p_min.z as i32);
        let (x_max, y_max, z_max) = (p_max.x as i32, p_max.y as i32, p_max.z as i32);

		for x in x_min .. x_max {
			for y in y_min .. y_max {
				for z in z_min .. z_max {
                   let vs_pos = vec3(x as f32, y as f32, z as f32);
                    let density = self.density(voxel_buffer.voxel_to_world(vs_pos));
					if density != Vec3::zero() {
                    	voxel_buffer.write(x, y, z, density);
					}
				}
			}
		}
	}
}
