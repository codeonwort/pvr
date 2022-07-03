pub mod constant;
pub mod voxel;
pub mod composite;

use crate::math::vec3::Vec3;
use crate::math::ray::Ray;
use crate::phasefn::PhaseFunction;

use std::marker::Sync;

// #todo-refactor
pub struct VolumeSample {
    pub emission: Vec3,         // #todo-physics: Physically correct unit
    pub absorption_coeff: Vec3, // [0.0 ~ 1.0]
    pub scattering_coeff: Vec3  // [0.0 ~ 1.0]
}
impl VolumeSample {
    fn new() -> VolumeSample {
        VolumeSample {
            emission: Vec3::zero(),
            absorption_coeff: Vec3::zero(),
            scattering_coeff: Vec3::zero()
        }
    }
}

pub trait Volume : Sync {
    // #todo-refactor: Wanna leave only sample() but secondary ray marching needs only absorption.
    // Using sample() for it degrades performance.
    fn emission(&self, p: Vec3) -> Vec3;
    fn absorption(&self, p: Vec3) -> Vec3;
    fn scattering(&self, p: Vec3) -> Vec3;
    fn sample(&self, world_position: Vec3) -> VolumeSample;

    fn set_phase_function(&mut self, phase_fn: Box<dyn PhaseFunction>);
    fn phase_function(&self, p: Vec3, wi: Vec3, wo: Vec3) -> f32;

    // Return valid intervals to raymarch given a ray
    fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)>; // (t_min, t_max) of the ray

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
