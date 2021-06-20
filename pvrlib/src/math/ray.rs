use super::vec3::Vec3;

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    pub o: Vec3, // origin
    pub d: Vec3  // direction
}

impl Ray {
    // CAUTION: You should normalize the direction yourself
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        Ray { o: origin, d: direction }
    }
    pub fn at(&self, t: f32) -> Vec3 {
        self.o + t * self.d
    }
}
