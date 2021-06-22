use crate::math::vec3::*;

// (1.0 / 4pi)
pub const ISOTROPIC_PHASE_FN: f32 = 1.0 / (4.0 * std::f32::consts::PI);

// #todo-phase: PhaseFunction could be an enum to avoid heap allocation and vtable,
// more performant less flexible. Can't decide which will be better, but switching
// between 
// Scattering prob. given incoming and outgoing directions.
pub trait PhaseFunction : Sync {
    fn probability(&self, wi: Vec3, wo: Vec3) -> f32;
}

pub struct Isotropic {}
impl PhaseFunction for Isotropic {
    fn probability(&self, _wi: Vec3, _wo: Vec3) -> f32 {
        ISOTROPIC_PHASE_FN
    }
}

// Similar to specular behavior in a surface BRDF
pub struct HenyeyGreenstein {
    // eccentricity parameter (-1.0 ~ 1.0)
    // g = 0 : isotropic
    // g > 0 : forward scattering
    // g < 0 : back scattering
    pub g: f32
}
impl PhaseFunction for HenyeyGreenstein {
    fn probability(&self, wi: Vec3, wo: Vec3) -> f32 {
        let g = self.g;
        let t = wi & wo;
        
        ISOTROPIC_PHASE_FN * (1.0 - g * g) /
            (1.0 + g * g - 2.0 * g * t).powf(1.5)
    }
}

// Can represent both diffuse and specular behaviors
pub struct DoubleHenyeyGreenstein {
    // DHG = b * HG(g1) + (1-b) * HG(g2)
    pub g1: f32,
    pub g2: f32,
    pub b: f32
}
impl PhaseFunction for DoubleHenyeyGreenstein {
    fn probability(&self, wi: Vec3, wo: Vec3) -> f32 {
        let g1 = self.g1;
        let g2 = self.g2;
        let b = self.b;
        let t = wi & wo;

        let hg1 = ISOTROPIC_PHASE_FN * (1.0 - g1 * g1) /
            (1.0 + g1 * g1 - 2.0 * g1 * t).powf(1.5);
        let hg2 = ISOTROPIC_PHASE_FN * (1.0 - g2 * g2) /
            (1.0 + g2 * g2 - 2.0 * g2 * t).powf(1.5);
        
        b * hg1 + (1.0 - b) * hg2
    }
}
