use super::*;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::math::aabb::AABB;
use crate::phasefn::PhaseFunction;
use crate::voxelbuffer::VoxelBuffer;

pub struct VoxelVolume {
    pub buffer: Box<dyn VoxelBuffer>,
    pub phase_fn: Box<dyn PhaseFunction>,

    // temp
    pub emission_value: Vec3,
    pub absorption_coeff: Vec3,

    // #todo-refactor: transform for VoxelVolume
    // pub transform: Transform
    pub world_bounds: AABB
}

impl VoxelVolume {
    pub fn get_buffer(&mut self) -> &mut dyn VoxelBuffer {
        &mut *self.buffer
    }

    pub fn world_to_voxel(&self, p: Vec3) -> Vec3 {
        fit(p, self.world_bounds.min, self.world_bounds.max, Vec3::zero(), self.buffer.get_sizef())
    }
    pub fn voxel_to_world(&self, p: Vec3) -> Vec3 {
        fit(p, Vec3::zero(), self.buffer.get_sizef(), self.world_bounds.min, self.world_bounds.max)
    }
    pub fn world_to_local(&self, world_position: Vec3) -> Vec3 {
        fit(world_position,
            self.world_bounds.min, self.world_bounds.max,
            Vec3::zero(), Vec3::one())
    }

    pub fn sample_by_world_position(&self, p: Vec3) -> Vec3 {
        let uvw = self.world_to_local(p);
        self.buffer.sample_by_local_position(uvw.x, uvw.y, uvw.z)
    }
}

// #todo-refactor: voxel buffer should use uniform coordinate, not world position
impl Volume for VoxelVolume {
    fn emission(&self, p: Vec3) -> Vec3 {
        self.emission_value * self.sample_by_world_position(p)
    }
    fn absorption(&self, p: Vec3) -> Vec3 {
        self.absorption_coeff * self.sample_by_world_position(p)
    }
    fn scattering(&self, p: Vec3) -> Vec3 {
        self.sample_by_world_position(p)
    }
    fn sample(&self, world_position : Vec3) -> VolumeSample {
        let density = self.sample_by_world_position(world_position);
        VolumeSample {
            emission: self.emission_value * density,
            absorption_coeff: self.absorption_coeff * density,
            scattering_coeff: density
        }
    }

    fn set_phase_function(&mut self, phase_fn: Box<dyn PhaseFunction>) {
        self.phase_fn = phase_fn;
    }
    fn phase_function(&self, _p: Vec3, wi: Vec3, wo: Vec3) -> f32 {
        self.phase_fn.probability(wi, wo)
    }

    fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)> {
        self.buffer.find_intersections(ray, self.world_bounds)
    }
    fn world_bounds(&self) -> AABB {
        self.world_bounds
    }
}
