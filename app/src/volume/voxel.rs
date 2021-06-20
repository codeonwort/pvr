use pvrlib::math::vec3::*;
use pvrlib::math::ray::*;
use pvrlib::voxelbuffer::VoxelBuffer;
use crate::volume::volume::*;

pub struct VoxelVolume {
    pub buffer: Box<dyn VoxelBuffer>,

    // temp
    pub emission_value: Vec3,
    pub absorption_coeff: Vec3,
}

impl VoxelVolume {
    pub fn get_buffer(&mut self) -> &mut dyn VoxelBuffer {
        &mut *self.buffer
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
        self.buffer.sample_by_world_position(p)
    }
    fn phase_function(&self, p: Vec3, wi: Vec3, wo: Vec3) -> f32 {
        // WTF logic for composite volume
        let den = self.buffer.sample_by_world_position(p).max_component();

        let t = wi & wo;
        let g = 0.76;
        
        den * ISOMORPHIC_PHASE_FN * (1.0 - g * g) /
            (1.0 + g * g - 2.0 * g * t).powf(1.5)
    }
    fn get_intersection(&self, ray: Ray) -> Vec<(f32, f32)> {
        let mut intervals = Vec::new();
        if let Some(v) = self.buffer.get_ws_bounds().intersect(ray) {
            intervals.push(v);
        }

        intervals
    }
}
