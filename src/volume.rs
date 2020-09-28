use crate::vec3::Vec3;
use crate::ray::Ray;

pub trait Volume {
    fn emission(&self, p: Vec3) -> Vec3;
    fn absorption(&self, p: Vec3) -> f32;
    fn get_intersection(&self, ray: Ray) -> Option<(f32, f32)>; // (t_min, t_max) of the ray
}

// todo-volume: Simple sphere for now
pub struct ConstantVolume {
    center: Vec3,
    radius: f32,

    emission_value: Vec3,
    absorption_coeff: f32
}

impl ConstantVolume {
    pub fn new(center: Vec3, radius: f32, emission: Vec3, absorption: f32) -> ConstantVolume {
        ConstantVolume { center: center, radius: radius, emission_value: emission, absorption_coeff: absorption }
    }

    fn contains(&self, p: Vec3) -> bool {
        (p - self.center).length_sq() <= (self.radius * self.radius)
    }
}

impl Volume for ConstantVolume {
    fn emission(&self, p: Vec3) -> Vec3 {
        if self.contains(p) { self.emission_value } else { Vec3::new(0.0, 0.0, 0.0) }
    }
    fn absorption(&self, p: Vec3) -> f32 {
        if self.contains(p) { self.absorption_coeff } else { 0.0 }
    }
    fn get_intersection(&self, ray: Ray) -> Option<(f32, f32)> {
        let delta = ray.o - self.center;
        let a = ray.d & ray.d;
        let b = 2.0 * (ray.d & delta);
        let c = (delta & delta) - (self.radius * self.radius);

        solve_quadratic(a, b, c)
    }
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
