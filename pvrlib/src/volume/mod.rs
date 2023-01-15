pub mod constant;
pub mod voxel;
pub mod composite;

use crate::math::vec3::vec3f;
use crate::math::ray::Ray;
use crate::math::aabb::AABB;
use crate::phasefn::PhaseFunction;

use std::marker::Sync;

pub struct VolumeSample {
    pub emission: vec3f,         // #todo-physics: Physically correct unit
    pub absorption_coeff: vec3f, // [0.0 ~ 1.0]
    pub scattering_coeff: vec3f  // [0.0 ~ 1.0]
}
impl VolumeSample {
    fn new() -> VolumeSample {
        VolumeSample {
            emission: vec3f::zero(),
            absorption_coeff: vec3f::zero(),
            scattering_coeff: vec3f::zero()
        }
    }
}

// Designed for physically based volumetric lighting.
// #note: Do not introduce the concept of 'density' here.
pub trait Volume : Sync {

    // ----------------------------------------------------------
    // Lighting properties

    /// Sample emission at the given position.
    fn emission(&self, world_position: vec3f) -> vec3f;

    /// Sample absorption coefficient at the given position.
    fn absorption_coeff(&self, world_position: vec3f) -> vec3f;

    /// Sample scattering coefficient at the given position.
    fn scattering_coeff(&self, world_position: vec3f) -> vec3f;

    /// Sample emission, absorption coeff, and scattering coeff at once.
    fn sample(&self, world_position: vec3f) -> VolumeSample;

    // #todo-refactor: Remove position parameter.
    // See CompositeVolume::phase_function() for why it exists.
    /// Evaluate phase function at `world_position`.<br/>
    /// `wi` : Incoming direction.<br/>
    /// `wo` : Outgoing direction.
    fn phase_function(&self, world_position: vec3f, wi: vec3f, wo: vec3f) -> f32;

    // #todo-refactor: This is not mandatory for trait API.
    fn set_phase_function(&mut self, phase_fn: Box<dyn PhaseFunction>);

    /// Return valid intervals to raymarch the given ray.
    fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)>; // (t_min, t_max) of the ray

    /// World space bounds of this volume.
    fn world_bounds(&self) -> AABB;

}
