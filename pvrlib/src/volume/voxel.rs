use super::*;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::phasefn::PhaseFunction;
use crate::voxelbuffer::VoxelBuffer;

pub struct VoxelVolume {
    pub buffer: Box<dyn VoxelBuffer>,
    pub phase_fn: Box<dyn PhaseFunction>,

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

    fn set_phase_function(&mut self, phase_fn: Box<dyn PhaseFunction>) {
        self.phase_fn = phase_fn;
    }
    fn phase_function(&self, p: Vec3, wi: Vec3, wo: Vec3) -> f32 {
        // #todo-phase: WTF logic for composite volume
        let den = self.buffer.sample_by_world_position(p).max_component();
        den * self.phase_fn.probability(wi, wo)
    }

    fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)> {
        self.buffer.find_intersections(ray)
    }
}
