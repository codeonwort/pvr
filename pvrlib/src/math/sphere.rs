use crate::math::vec3::*;
use crate::math::ray::*;

// Forked from: https://www.shadertoy.com/view/XtBXDw

#[derive(Clone, Copy, Debug)]
pub struct Sphere {
    pub origin: vec3f,
    pub radius: f32
}

impl Default for Sphere {
    // Unit sphere
    fn default() -> Self {
        Sphere { origin: vec3f::zero(), radius: 1.0 }
    }
}

pub struct SphereHit {
    pub valid: bool,
    pub ray_t: f32,   // ray time
    pub normal: vec3f, // normal at hit point
    pub origin: vec3f  // location of hit point
}

impl Sphere {
    pub fn intersect(&self, ray: Ray) -> SphereHit {
        let rc = self.origin - ray.o;
        let r2 = self.radius * self.radius;
        let tca = rc.dot(ray.d);

        let d2 = rc.dot(rc) - tca * tca;
        if d2 > r2 {
            return SphereHit::no_hit();
        }

        let thc = (r2 - d2).sqrt();
        let mut t0 = tca - thc;
        let t1 = tca + thc;

        if t0 < 0.0 { t0 = t1; }
        
        let impact = ray.o + ray.d * t0;

        SphereHit {
            valid: true,
            ray_t: t0,
            normal: (impact - self.origin) / self.radius,
            origin: impact
        }
    }
}

impl SphereHit {
    fn no_hit() -> SphereHit {
        SphereHit {
            valid: false, ray_t: -1.0, normal: vec3(0.0, 0.0, 1.0), origin: vec3(0.0, 0.0, 0.0)
        }
    }
}