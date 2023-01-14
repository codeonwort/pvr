use super::*;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::math::solve_quadratic;
use crate::math::aabb::AABB;
use crate::phasefn::PhaseFunction;

pub enum ConstantVolumeShape {
    Box,
    Sphere
}

pub struct ConstantVolume {
    shape: ConstantVolumeShape,
    center: vec3f,
    radius: f32,
    box_bounds: AABB,

    emission_value: vec3f,
    absorption_coeff: vec3f,
    scattering_coeff: vec3f,

    phase_fn: Box<dyn PhaseFunction>
}

impl ConstantVolume {
    pub fn new(
        shape: ConstantVolumeShape,
        center: vec3f,
        radius: f32,
        emission: vec3f,
        absorption: vec3f,
        scattering: vec3f,
        phase_fn: Box<dyn PhaseFunction>) -> ConstantVolume
    {
        let r = vec3(radius, radius, radius);
        ConstantVolume {
            shape: shape,
            center: center,
            radius: radius,
            box_bounds: AABB { min: center - r, max: center + r },
            emission_value: emission,
            absorption_coeff: absorption,
            scattering_coeff: scattering,
            phase_fn: phase_fn
        }
    }

    fn contains(&self, p: vec3f) -> bool {
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

    fn aabb_vs_ray(&self, ray: Ray) -> Vec<(f32, f32)> {
        let mut t_min = f32::MIN;
        let mut t_max = f32::MAX;
        let mut hit = true;
        for i in 0..3 {
            let inv_d = 1.0 / ray.d[i];
            let mut t0 = (self.box_bounds.min[i] - ray.o[i]) * inv_d;
            let mut t1 = (self.box_bounds.max[i] - ray.o[i]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max < t_min {
                hit = false;
            }
        }

        if hit {
            vec!((t_min, t_max))
        } else {
            Vec::new()
        }
    }

    fn sphere_vs_ray(&self, ray: Ray) -> Vec<(f32, f32)> {
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

impl Volume for ConstantVolume {
    fn emission(&self, p: vec3f) -> vec3f {
        if self.contains(p) { self.emission_value } else { vec3f::zero() }
    }
    fn absorption_coeff(&self, p: vec3f) -> vec3f {
        if self.contains(p) { self.absorption_coeff } else { vec3f::zero() }
    }
    fn scattering_coeff(&self, p: vec3f) -> vec3f {
        if self.contains(p) { self.scattering_coeff } else { vec3f::zero() }
    }
    fn sample(&self, world_position: vec3f) -> VolumeSample {
        // #todo: Hmm... this should not be called if world_position is out of bounds at first.
        if self.contains(world_position) {
            return VolumeSample {
                emission: self.emission_value,
                absorption_coeff: self.absorption_coeff,
                scattering_coeff: self.scattering_coeff
            };
        } else {
            return VolumeSample {
                emission: vec3f::zero(),
                absorption_coeff: vec3f::zero(),
                scattering_coeff: vec3f::zero()
            };
        }
    }

    fn set_phase_function(&mut self, phase_fn: Box<dyn PhaseFunction>) {
        self.phase_fn = phase_fn;
    }
    fn phase_function(&self, p: vec3f, wi: vec3f, wo: vec3f) -> f32 {
        if self.contains(p) {
            self.phase_fn.probability(wi, wo)
        } else {
            0.0
        }
    }

    fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)> {
        match self.shape {
            ConstantVolumeShape::Box => self.aabb_vs_ray(ray),
            ConstantVolumeShape::Sphere => self.sphere_vs_ray(ray)
        }
    }

    fn world_bounds(&self) -> AABB {
        let r = vec3f::new(self.radius, self.radius, self.radius);
        AABB {
            min: self.center - r,
            max: self.center + r
        }
    }
}
