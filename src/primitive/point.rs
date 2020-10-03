use crate::vec3::*;
use crate::voxel::*;
use crate::primitive::primitive::*;

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

        println!("Rasterize a Point into voxel buffer: bounds={{min: {:?}, max: {:?}}}", p_min, p_max);

        for x in x_min .. x_max {
            for y in y_min .. y_max {
                for z in z_min .. z_max {
                    let vs_pos = vec3(x as f32, y as f32, z as f32);
                    let density = self.density(voxel_buffer.voxel_to_world(vs_pos));
                    
                    voxel_buffer.write(x, y, z, density);
                }
            }
        }
    }
}
