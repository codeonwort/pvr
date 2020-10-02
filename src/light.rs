use crate::vec3::*;

pub struct LightSample {
    pub luminance: Vec3,
    pub position: Vec3
}

pub trait Light {
    // Luminance arriving at a given world position, ignoring occlusion.
    fn sample(&self, ray_position: Vec3, ray_direction: Vec3) -> LightSample;
}

pub struct PointLight {
    pub position: Vec3,
    pub intensity: Vec3
}

impl Light for PointLight {
    fn sample(&self, ray_position: Vec3, ray_direction: Vec3) -> LightSample {
        let falloff = 1.0 / (ray_position - self.position).length_sq();
        
        LightSample { luminance: self.intensity * falloff, position: self.position }
    }
}
