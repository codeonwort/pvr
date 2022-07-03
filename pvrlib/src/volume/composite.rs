use super::Volume;
use super::VolumeSample;
use crate::math::vec3::*;
use crate::math::ray::*;
use crate::math::aabb::AABB;
use crate::phasefn::PhaseFunction;

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

    fn sample(&self, world_position: Vec3) -> VolumeSample {
        let mut samp = VolumeSample::new();

        // #todo: What if childrens intersect each other
        for child in &self.children {
            let child_samp = child.sample(world_position);
            samp.emission += child_samp.emission;
            samp.absorption_coeff += child_samp.absorption_coeff;
            samp.scattering_coeff += child_samp.scattering_coeff;
        }

        return samp;
    }

    fn set_phase_function(&mut self, _phase_fn: Box<dyn PhaseFunction>) {
        // #todo-phase: What to do here?
        println!("WARNING: set_phase_fn() on CompositeVolume won't do nothing");
    }
    fn phase_function(&self, p: Vec3, wi: Vec3, wo: Vec3) -> f32 {
        // #todo-phase: Assumes no overlap between child volumes.
        // Need to introduce weight per phase fn.
        let mut total_p = 0.0;
        for child in &self.children {
            total_p += child.phase_function(p, wi, wo);
        }

        total_p
    }

    fn find_intersections(&self, ray: Ray) -> Vec<(f32, f32)> {
        let mut intervals = Vec::new();
        for child in &self.children {
            intervals.append(&mut child.find_intersections(ray));
        }

        intervals
    }

    fn world_bounds(&self) -> AABB {
        if self.children.len() == 0 {
            return AABB {
                min: Vec3::zero(), 
                max: Vec3::zero()
            };
        }
        let mut aabb = self.children[0].world_bounds();
        let len = self.children.len();
        for _i in 1..len {
            aabb = aabb.extend(self.children[1].world_bounds());
        }

        aabb
    }
}
