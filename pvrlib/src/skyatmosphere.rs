
use crate::math::vec3::*;
use crate::math::ray::Ray;

pub struct SkyAtmosphere {
    is_empty: bool,
    sun_direction: Vec3 // Sun incoming direction
}

impl SkyAtmosphere {
    // Static methods
    pub fn new_empty() -> SkyAtmosphere {
        SkyAtmosphere {
            is_empty: true,
            sun_direction: vec3(0.0, -1.0, 0.0)
        }
    }
    pub fn new_atmosphere(sun_direction: Vec3) -> SkyAtmosphere {
        SkyAtmosphere {
            is_empty: false,
            sun_direction: sun_direction
        }
    }

    pub fn sample(&self, ray: Ray) -> Vec3 {
        if self.is_empty {
            return vec3(0.0, 0.0, 0.0);
        }
        
        // #todo-sky: Atmospheric scattering
        vec3(0.0, 0.0, 1.0)
    }
}
