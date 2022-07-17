use crate::math::vec3::*;
use crate::primitive::{Primitive, RasterizationPrimitive};
use crate::volume::voxel::VoxelVolume;

use crate::math::noise::*; // pyroclastic test

// A simple point with a pyroclastic fractal function attached.
// Used for billowing smoke, cumulus-type cloud formations, etc.
pub struct PyroclasticPoint {
    pub center: Vec3,
    pub radius: f32
}

impl PyroclasticPoint {
    fn density(&self, p: Vec3) -> Vec3 {
        let len = (self.center - p).length_sq();
        if len <= (self.radius * self.radius) {
            Vec3::one()
        } else {
            Vec3::zero()
        }
    }
}

impl Primitive for PyroclasticPoint {
	//
}

impl RasterizationPrimitive for PyroclasticPoint {
    fn rasterize(&self, voxel_volume: &mut VoxelVolume) {
        let p_min: Vec3 = voxel_volume.world_to_voxel(self.center - self.radius.into());
        let p_max: Vec3 = voxel_volume.world_to_voxel(self.center + self.radius.into());
        let (x_min, y_min, z_min) = (p_min.x as i32, p_min.y as i32, p_min.z as i32);
        let (x_max, y_max, z_max) = (p_max.x as i32, p_max.y as i32, p_max.z as i32);

        println!("Rasterize a PyroclasticPoint: vs_bounds={{min: {:?}, max: {:?}}}", p_min, p_max);

        //let mut rng = crate::math::random::MT19937::new(0);

        for x in x_min .. x_max {
            for y in y_min .. y_max {
                for z in z_min .. z_max {
                    let vs_pos = vec3(x as f32, y as f32, z as f32);
                    let density = self.density(voxel_volume.voxel_to_world(vs_pos));

                    let ws_pos = voxel_volume.voxel_to_world(vs_pos);
                    let ls_pos = (ws_pos - self.center) / self.radius;

                    // #todo-noise: From the book's source code, but looks weird
                    //let amplitude = 50.0;
                    //let mut ns_pos = ls_pos;
                    //ns_pos.x += rng.rand_range(-100.0, 100.0) as f32;
                    //ns_pos.y += rng.rand_range(-100.0, 100.0) as f32;
                    //ns_pos.z += rng.rand_range(-100.0, 100.0) as f32;
                    //let noise = fBm(ns_pos) * amplitude;
                    //let is_pyroclastic = true; // Parameterize this
                    //let pyro;
                    //if is_pyroclastic {
                    //    let sphere_func = ls_pos.length() - 1.0;
                    //    let filter_width = voxel_volume.world_bounds.size().length() / self.radius;
                    //    pyro = pyroclastic(sphere_func, noise, filter_width);
                    //} else {
                    //    let distance_func = 1.0 - ls_pos.length();
                    //    pyro = (distance_func + noise).max(0.0);
                    //}

                    // My pyroclastic
                    let noise = fBm(8.0 * ls_pos);
                    let sphere_func = ls_pos.length() + 0.9;
                    let filter_width = 2.0;//ws_bounds.size().length() / self.radius;
                    let pyro = pyroclastic(sphere_func, noise, filter_width);

					if density != Vec3::zero() {
	                    voxel_volume.get_buffer().write(x, y, z, density * pyro);
					}
                }
            }
        }
    }
}
