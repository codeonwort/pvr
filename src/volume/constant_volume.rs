use crate::volume::volume::*;
use crate::vec3::*;
use crate::ray::*;

// todo-volume: Simple sphere for now
pub struct ConstantVolume {
    center: Vec3,
    radius: f32,

    emission_value: Vec3,
    absorption_coeff: Vec3
}

impl ConstantVolume {
    pub fn new(center: Vec3, radius: f32, emission: Vec3, absorption: Vec3) -> ConstantVolume {
        ConstantVolume { center: center, radius: radius, emission_value: emission, absorption_coeff: absorption }
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
    fn scattering(&self, p: Vec3) -> Vec3 {
        Vec3::one()
    }
    fn phase_function(&self, wi: Vec3, wo: Vec3) -> f32 {
        ISOMORPHIC_PHASE_FN
    }
    fn get_intersection(&self, ray: Ray) -> Option<(f32, f32)> {
        let delta = ray.o - self.center;
        let a = ray.d & ray.d;
        let b = 2.0 * (ray.d & delta);
        let c = (delta & delta) - (self.radius * self.radius);

        solve_quadratic(a, b, c)
    }
}