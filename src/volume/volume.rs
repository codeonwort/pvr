use crate::vec3::Vec3;
use crate::ray::Ray;

use std::marker::Sync;

// #todo: Support various phase functions
//pub type PhaseFunction = fn(wi: Vec3, wo: Vec3) -> f32;

pub const ISOMORPHIC_PHASE_FN: f32 = 1.0 / (4.0 * std::f32::consts::PI);

pub trait Volume : Sync {
    // coefficients
    fn emission(&self, p: Vec3) -> Vec3;
    fn absorption(&self, p: Vec3) -> Vec3;
    fn scattering(&self, p: Vec3) -> Vec3;
    fn phase_function(&self, p: Vec3, wi: Vec3, wo: Vec3) -> f32;

    fn get_intersection(&self, ray: Ray) -> Vec<(f32, f32)>; // (t_min, t_max) of the ray
}

// https://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
pub fn solve_quadratic(a: f32, b: f32, c: f32) -> Option<(f32, f32)> {
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
