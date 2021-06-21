// #todo-light: Make submodule if light codebase goes bigger

use crate::math::vec3::*;

use std::marker::Sync;

pub struct LightSample {
    pub luminance: Vec3, // #todo-light: Luminance? not radiance?
    pub position: Vec3
}

pub trait Light : Sync {
    // Luminance arriving at a given world position, ignoring occlusion.
    fn sample(&self, ray_position: Vec3, ray_direction: Vec3) -> LightSample;
}

pub struct PointLight {
    pub position: Vec3,
    pub intensity: Vec3 // #todo-light: What? Give it a physical meaning
}

impl Light for PointLight {
    fn sample(&self, ray_position: Vec3, _ray_direction: Vec3) -> LightSample {
        let len_sq = (ray_position - self.position).length_sq();

        if len_sq < 1.0 {
            LightSample { luminance: self.intensity, position: self.position }
        } else {
            let falloff = 1.0 / len_sq;
        
            LightSample { luminance: self.intensity * falloff, position: self.position }
        }
    }
}
