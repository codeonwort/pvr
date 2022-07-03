use super::*;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::phasefn::PhaseFunction;

pub enum ConstantVolumeShape {
    Box,
    Sphere
}

// #todo-volume: Simple sphere for now
pub struct ConstantVolume {
    shape: ConstantVolumeShape,
    center: Vec3,
    radius: f32,

    emission_value: Vec3,
    absorption_coeff: Vec3,
    scattering_coeff: Vec3,

    phase_fn: Box<dyn PhaseFunction>
}

impl ConstantVolume {
    pub fn new(
        shape: ConstantVolumeShape,
        center: Vec3,
        radius: f32,
        emission: Vec3,
        absorption: Vec3,
        scattering: Vec3,
        phase_fn: Box<dyn PhaseFunction>) -> ConstantVolume
    {
        ConstantVolume {
            shape: shape,
            center: center,
            radius: radius,
            emission_value: emission,
            absorption_coeff: absorption,
            scattering_coeff: scattering,
            phase_fn: phase_fn
        }
    }

    fn contains(&self, p: Vec3) -> bool {
        match self.shape {
            ConstantVolumeShape::Box => {
                let sides = p - self.center;
                return sides.x.abs() <= self.radius
                    && sides.y.abs() <= self.radius
                    && sides.z.abs() <= self.radius;
            }
            ConstantVolumeShape::Sphere => {
                (p - self.center).length_sq() <= (self.radius * self.radius)
            }
        }
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
    fn sample(&self, world_position: Vec3) -> VolumeSample {
        // #todo: Hmm... this should not be called if world_position is out of bounds at first.
        if self.contains(world_position) {
            return VolumeSample {
                emission: self.emission_value,
                absorption_coeff: self.absorption_coeff,
                scattering_coeff: self.scattering_coeff
            };
        } else {
            return VolumeSample {
                emission: Vec3::zero(),
                absorption_coeff: Vec3::zero(),
                scattering_coeff: Vec3::zero()
            };
        }
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
