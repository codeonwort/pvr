// #todo-light: Make submodule if light codebase goes bigger

use crate::math::vec3::*;

use std::marker::Sync;

pub struct LightSample {
    pub luminance: vec3f, // #todo-light: Luminance? not radiance?
    pub position: vec3f
}

pub trait Light : Sync {
    // Luminance arriving at a given world position, ignoring occlusion.
    fn sample(&self, ray_position: vec3f, ray_direction: vec3f) -> LightSample;
}

pub struct PointLight {
    pub position: vec3f,
    pub intensity: vec3f // #todo-light: What? Give it a physical meaning
}

impl Light for PointLight {
    fn sample(&self, ray_position: vec3f, _ray_direction: vec3f) -> LightSample {
        let len_sq = (ray_position - self.position).length_sq();

        if len_sq < 1.0 {
            LightSample { luminance: self.intensity, position: self.position }
        } else {
            let falloff = 1.0 / len_sq;
        
            LightSample { luminance: self.intensity * falloff, position: self.position }
        }
    }
}
