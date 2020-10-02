use crate::vec3::*;

#[derive(Copy, Clone, Default, Debug)]
pub struct AABB {
    pub min: Vec3,
    pub max: Vec3
}

impl AABB {
    pub fn center(&self) -> Vec3 {
        0.5 * (self.min + self.max)
    }
    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }
    pub fn half_size(&self) -> Vec3 {
        0.5 * (self.max - self.min)
    }
}
