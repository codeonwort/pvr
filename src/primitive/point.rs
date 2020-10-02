use crate::vec3::*;
use crate::voxel::*;
use crate::primitive::primitive::*;

pub struct Point {
    pub center: Vec3,
    pub radius: f32
}

impl Point {
    fn density(&self, vs_pos: Vec3) -> Vec3 {
        let len = (self.center - vs_pos).length_sq();
        if len <= (self.radius * self.radius) {
            Vec3::one()
        } else {
            Vec3::zero()
        }
    }
}

impl Primitive for Point {
    fn rasterize(&self, voxel_buffer: &mut VoxelBuffer) {
        let voxel_res = voxel_buffer.get_sizef();
        let ws_bounds = voxel_buffer.get_ws_bounds();

        let mut p_max: Vec3 = self.center + self.radius.into();
        let mut p_min: Vec3 = self.center - self.radius.into();
        p_max = fit(p_max, ws_bounds.min, ws_bounds.max, Vec3::zero(), voxel_res);
        p_min = fit(p_min, ws_bounds.min, ws_bounds.max, Vec3::zero(), voxel_res);

        let (x_max, y_max, z_max) = (p_max.x as i32, p_max.y as i32, p_max.z as i32);
        let (x_min, y_min, z_min) = (p_min.x as i32, p_min.y as i32, p_min.z as i32);

        println!("max:{:?} min:{:?}", p_max, p_min);

        for x in x_min .. x_max {
            for y in y_min .. y_max {
                for z in z_min .. z_max {
                    let vs_pos = vec3(x as f32, y as f32, z as f32);
                    let density = self.density(vs_pos);

                    voxel_buffer.write(x, y, z, density);
                }
            }
        }
    }
}
