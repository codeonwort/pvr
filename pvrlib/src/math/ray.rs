use super::vec3::vec3f;

#[derive(Copy, Clone, Default, Debug)]
pub struct Ray {
    pub o: vec3f, // origin
    pub d: vec3f  // direction
}

impl Ray {
    // CAUTION: You should normalize the direction yourself
    pub fn new(origin: vec3f, direction: vec3f) -> Ray {
        Ray { o: origin, d: direction }
    }
    pub fn at(&self, t: f32) -> vec3f {
        self.o + t * self.d
    }
}
