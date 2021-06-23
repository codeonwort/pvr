use super::*;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::phasefn::PhaseFunction;

// todo-volume: Simple sphere for now
pub struct ConstantVolume {
    center: Vec3,
    radius: f32,

    emission_value: Vec3,
    absorption_coeff: Vec3,

    phase_fn: Box<dyn PhaseFunction>
}

impl ConstantVolume {
    pub fn new(center: Vec3, radius: f32, emission: Vec3, absorption: Vec3, phase_fn: Box<dyn PhaseFunction>) -> ConstantVolume {
        ConstantVolume {
            center: center,
            radius: radius,
            emission_value: emission,
            absorption_coeff: absorption,
            phase_fn: phase_fn
        }
    }

    fn contains(&self, p: Vec3) -> bool {
        (p - self.center).length_sq() <= (self.radius * self.radius)
    }
}

impl Volume for ConstantVolume {
    fn emission(&self, p: Vec3) -> Vec3 {
        if self.contains(p) { self.emission_value } else { Vec3::zero() }
    }
    fn absorption(&self, p: Vec3) -> Vec3 {
        if self.contains(p) { self.absorption_coeff } else { Vec3::zero() }
    }
    fn scattering(&self, _p: Vec3) -> Vec3 {
        Vec3::one()
    }

    fn set_phase_function(&mut self, phase_fn: Box<dyn PhaseFunction>) {
        self.phase_fn = phase_fn;
    }
    fn phase_function(&self, p: Vec3, wi: Vec3, wo: Vec3) -> f32 {
        if self.contains(p) {
            self.phase_fn.probability(wi, wo)
        } else {
            0.0
        }
    }

    fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)> {
        let delta = ray.o - self.center;
        let a = ray.d & ray.d;
        let b = 2.0 * (ray.d & delta);
        let c = (delta & delta) - (self.radius * self.radius);

        let mut intervals = Vec::new();
        if let Some(v) = solve_quadratic(a, b, c) {
            intervals.push(v);
        }

        intervals
    }
}
