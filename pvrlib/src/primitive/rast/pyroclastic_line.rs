use crate::math::vec3::*;
use crate::math::noise::{fBm, pyroclastic};
use crate::primitive::{Primitive, RasterizationPrimitive};
use crate::volume::voxel::VoxelVolume;

pub struct PyroclasticLine {
    pub	p0: vec3f,   // World position of a vertex
    pub	p1: vec3f,   // World position of another vertex
    pub	radius: f32 // Line thickness
}

impl PyroclasticLine {
    pub fn density(&self, p: vec3f) -> vec3f {
        let dist = self.closest_distance(p);
        if dist <= self.radius {
            vec3f::one()
        } else {
            vec3f::zero()
        }
    }
    fn closest_distance(&self, world_position: vec3f) -> f32 {
        let u = self.p1 - self.p0;
        u.cross(world_position - self.p0).length() / u.length()
    }
}

impl Primitive for PyroclasticLine {}

impl RasterizationPrimitive for PyroclasticLine {
    fn rasterize(&self, voxel_volume: &mut VoxelVolume) {
        let p0 = voxel_volume.world_to_voxel(self.p0);
        let p1 = voxel_volume.world_to_voxel(self.p1);
        let aug = vec3(self.radius, self.radius, self.radius);

        let p_min = vec3(f32::min(p0.x, p1.x), f32::min(p0.y, p1.y), f32::min(p0.z, p1.z)) - aug;
        let p_max = vec3(f32::max(p0.x, p1.x), f32::max(p0.y, p1.y), f32::max(p0.z, p1.z)) + aug;
        let (x_min, y_min, z_min) = (p_min.x as i32, p_min.y as i32, p_min.z as i32);
        let (x_max, y_max, z_max) = (p_max.x as i32, p_max.y as i32, p_max.z as i32);

        println!("Rasterize a pyroclastic line: min=({}, {}, {}) max=({}, {}, {})",
            x_min, y_min, z_min, x_max, y_max, z_max);

        for x in x_min .. x_max {
            for y in y_min .. y_max {
                for z in z_min .. z_max {
                    let vs_pos = vec3(x as f32, y as f32, z as f32);
                    let density = self.density(voxel_volume.voxel_to_world(vs_pos));

                    let ws_pos = voxel_volume.voxel_to_world(vs_pos);
                    let ls_pos = voxel_volume.world_to_local(ws_pos);
                    let noise = fBm(16.0 * ls_pos);

                    let sphere_func = 0.5 + self.closest_distance(ws_pos) / self.radius;
                    let filter_width = 1.0;
                    let pyro = pyroclastic(sphere_func, noise, filter_width);

                    if density != vec3f::zero() {
                        let v = voxel_volume.get_buffer().read(x, y, z);
                        voxel_volume.get_buffer().write(x, y, z, v + density * pyro);
                    }
                }
            }
        }
    }
}
