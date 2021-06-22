use crate::math::vec3::*;

pub const ISOMORPHIC_PHASE_FN: f32 = 1.0 / (4.0 * std::f32::consts::PI);

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
        ISOMORPHIC_PHASE_FN
    }
}

pub struct HenyeyGreenstein {
    pub g: f32 // eccentricity parameter (default: 0.76)
}
impl PhaseFunction for HenyeyGreenstein {
    fn probability(&self, wi: Vec3, wo: Vec3) -> f32 {
        let g = self.g;
        let t = wi & wo;
        
        ISOMORPHIC_PHASE_FN * (1.0 - g * g) /
            (1.0 + g * g - 2.0 * g * t).powf(1.5)
    }
}
