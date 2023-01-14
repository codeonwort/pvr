use crate::math::vec3::*;
use crate::primitive::*;

pub struct Point {
    pub center: vec3f,
    pub radius: f32
}

impl Point {
    fn density(&self, p: vec3f) -> vec3f {
        let len = (self.center - p).length_sq();
        if len <= (self.radius * self.radius) {
            vec3f::one()
        } else {
            vec3f::zero()
        }
    }
}

impl Primitive for Point {
    //
}

impl RasterizationPrimitive for Point {
    fn rasterize(&self, voxel_volume: &mut VoxelVolume) {
        let p_min: vec3f = voxel_volume.world_to_voxel(self.center - self.radius.into());
        let p_max: vec3f = voxel_volume.world_to_voxel(self.center + self.radius.into());
        let (x_min, y_min, z_min) = (p_min.x as i32, p_min.y as i32, p_min.z as i32);
        let (x_max, y_max, z_max) = (p_max.x as i32, p_max.y as i32, p_max.z as i32);

        println!("Rasterize a Point: vs_bounds={{min: {:?}, max: {:?}}}", p_min, p_max);

        for x in x_min .. x_max {
            for y in y_min .. y_max {
                for z in z_min .. z_max {
                    let vs_pos = vec3(x as f32, y as f32, z as f32);
                    let density = self.density(voxel_volume.voxel_to_world(vs_pos));
                    if density != vec3f::zero() {
                        voxel_volume.get_buffer().write(x, y, z, density);
                    }
                }
            }
        }
    }
}
