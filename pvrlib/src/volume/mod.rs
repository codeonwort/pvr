pub mod constant;
pub mod voxel;
pub mod composite;

use crate::math::vec3::vec3f;
use crate::math::ray::Ray;
use crate::math::aabb::AABB;
use crate::phasefn::PhaseFunction;

use std::marker::Sync;

// #todo-refactor
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

pub trait Volume : Sync {
    // #todo-refactor: Wanna leave only sample() but secondary ray marching needs only absorption.
    // Using sample() for it degrades performance.
    fn emission(&self, p: vec3f) -> vec3f;
    fn absorption(&self, p: vec3f) -> vec3f;
    fn scattering(&self, p: vec3f) -> vec3f;
    // Sample emission, absorption coeff, and scattering coeff at once.
    fn sample(&self, world_position: vec3f) -> VolumeSample;

    fn set_phase_function(&mut self, phase_fn: Box<dyn PhaseFunction>);

    // #todo-refactor: Remove position parameter.
    // See CompositeVolume::phase_function() for why it exists.
    fn phase_function(&self, p: vec3f, wi: vec3f, wo: vec3f) -> f32;

    // Return valid intervals to raymarch given a ray
    fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)>; // (t_min, t_max) of the ray

    fn world_bounds(&self) -> AABB;
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
    let det = (b * b) - (4.0 * a * c);
    if det < 0.0 {
        None
    } else if det == 0.0 {
        let x = -0.5 * b / a;
        Some((x, x))
    } else {
        let q = if b > 0.0 { -0.5 * (b + det.sqrt()) } else { -0.5 * (b - det.sqrt()) };
        let x0 = q / a;
        let x1 = c / q;
        if x0 < x1 {
            Some((x0, x1))
        } else {
            Some((x1, x0))
        }
    }
}
