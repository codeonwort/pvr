use crate::vec3::*;
use crate::voxel::*;
use crate::primitive::primitive::*;

use crate::noise::*; // pyroclastic test

pub struct Point {
    pub center: Vec3,
    pub radius: f32
}

impl Point {
    fn density(&self, p: Vec3) -> Vec3 {
        let len = (self.center - p).length_sq();
        if len <= (self.radius * self.radius) {
            Vec3::one()
        } else {
            Vec3::zero()
        }
    }
}

impl Primitive for Point {
    fn rasterize(&self, voxel_buffer: &mut VoxelBuffer) {
        let p_min: Vec3 = voxel_buffer.world_to_voxel(self.center - self.radius.into());
        let p_max: Vec3 = voxel_buffer.world_to_voxel(self.center + self.radius.into());
        let (x_min, y_min, z_min) = (p_min.x as i32, p_min.y as i32, p_min.z as i32);
        let (x_max, y_max, z_max) = (p_max.x as i32, p_max.y as i32, p_max.z as i32);
        let ws_bounds = voxel_buffer.get_ws_bounds();

        println!("Rasterize a Point into voxel buffer: vs_bounds={{min: {:?}, max: {:?}}}", p_min, p_max);

        for x in x_min .. x_max {
            for y in y_min .. y_max {
                for z in z_min .. z_max {
                    let vs_pos = vec3(x as f32, y as f32, z as f32);
                    let density = self.density(voxel_buffer.voxel_to_world(vs_pos));

                    // pyroclastic test
                    let ws_pos = voxel_buffer.voxel_to_world(vs_pos);
                    let ls_pos = (ws_pos - self.center) / self.radius;
                    let noise = fBm(ls_pos * 20.0, 4, 0.5, 1.92);
                    
                    let sphere_func = ls_pos.length() - 1.0;
                    let filter_width = ws_bounds.size().length() / self.radius;
                    let pyro = pyroclastic(sphere_func, noise, filter_width);
                    
                    voxel_buffer.write(x, y, z, density * pyro);
                }
            }
        }
    }
}
