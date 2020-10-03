use crate::volume::volume::*;
use crate::vec3::*;
use crate::ray::*;
use crate::voxel::*;

pub struct VoxelVolume {
    pub buffer: VoxelBuffer, 

    // temp
    pub emission_value: Vec3,
    pub absorption_coeff: Vec3,
}

impl VoxelVolume {
    pub fn get_buffer(&mut self) -> &mut VoxelBuffer {
        &mut self.buffer
    }
}

impl Volume for VoxelVolume {
    fn emission(&self, p: Vec3) -> Vec3 {
        self.emission_value * self.buffer.sample_by_world_position(p)
    }
    fn absorption(&self, p: Vec3) -> Vec3 {
        self.absorption_coeff * self.buffer.sample_by_world_position(p)
    }
    fn scattering(&self, p: Vec3) -> Vec3 {
        Vec3::one()
    }
    fn phase_function(&self, wi: Vec3, wo: Vec3) -> f32 {
        ISOMORPHIC_PHASE_FN
    }
    fn get_intersection(&self, ray: Ray) -> Option<(f32, f32)> {
        self.buffer.get_ws_bounds().intersect(ray)
    }
}
