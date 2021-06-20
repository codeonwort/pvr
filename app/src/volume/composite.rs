use super::volume::Volume;
use pvrlib::math::vec3::*;
use pvrlib::math::ray::*;

pub struct CompositeVolume {
    pub children: Vec<Box<dyn Volume>>
}

impl CompositeVolume {
    //
}

impl Volume for CompositeVolume {
    
    fn emission(&self, p: Vec3) -> Vec3 {
        let mut total_emission = Vec3::zero();
        for child in &self.children {
            total_emission += child.emission(p);
        }

        total_emission
    }

    fn absorption(&self, p: Vec3) -> Vec3 {
        let mut total_absorption = Vec3::zero();
        for child in &self.children {
            total_absorption += child.absorption(p);
        }

        total_absorption
    }

    fn scattering(&self, p: Vec3) -> Vec3 {
        let mut total_scattering = Vec3::zero();
        for child in &self.children {
            total_scattering += child.scattering(p);
        }

        total_scattering
    }

    // #todo: This assumes no overlap between child volumes.
    fn phase_function(&self, p: Vec3, wi: Vec3, wo: Vec3) -> f32 {
        let mut total_p = 0.0;
        for child in &self.children {
            total_p += child.phase_function(p, wi, wo);
        }

        total_p
    }

    fn get_intersection(&self, ray: Ray) -> Vec<(f32, f32)> {
        let mut intervals = Vec::new();
        for child in &self.children {
            intervals.append(&mut child.get_intersection(ray));
        }

        intervals
    }
}
