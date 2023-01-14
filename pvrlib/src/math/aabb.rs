use super::vec3::*;
use super::ray::*;
use std::mem::swap;

#[derive(Copy, Clone, Default, Debug)]
pub struct AABB {
    pub min: vec3f,
    pub max: vec3f
}

impl AABB {
    pub fn center(&self) -> vec3f {
        0.5 * (self.min + self.max)
    }
    pub fn size(&self) -> vec3f {
        self.max - self.min
    }
    // Extent
    pub fn half_size(&self) -> vec3f {
        0.5 * (self.max - self.min)
    }
    // (ray_t_min, ray_t_max)
    pub fn intersect(&self, ray: Ray) -> Option<(f32, f32)> {
        let mut t_near = -std::f32::MAX;
        let mut t_far = std::f32::MAX;
        let epsilon = 1.0e-6;

        for dim in 0..3 {
            if ray.d[dim].abs() < epsilon {
                if ray.o[dim] < self.min[dim] || ray.o[dim] > self.max[dim] {
                    return None
                }
            }
            let mut t0 = (self.min[dim] - ray.o[dim]) / ray.d[dim];
            let mut t1 = (self.max[dim] - ray.o[dim]) / ray.d[dim];
            if t0 > t1 {
                swap(&mut t0, &mut t1);
            }
            t_near = if t_near > t0 { t_near } else { t0 };
            t_far = if t_far < t1 { t_far } else { t1 };
            if t_near > t_far || t_far < 0.0 {
                return None
            }
        }

        Some((t_near, t_far))
    }
    // Minimum bounds that encompasses original AABBs
    pub fn extend(&self, other: AABB) -> AABB {
        AABB {
            min: vec3f::min(self.min, other.min),
            max: vec3f::max(self.max, other.max)
        }
    }
}
