use crate::math::vec3::vec3f;
use crate::math::ray::Ray;

pub struct Camera {
    position: vec3f,
    // precalculation
    top_left: vec3f,
    horizontal: vec3f,
    vertical: vec3f,
    // basis
    _u: vec3f,
    _v: vec3f,
    _w: vec3f,
}

impl Camera {
    pub fn new(position: vec3f, look_at: vec3f, up: vec3f, fov_y: f32, aspect_ratio: f32) -> Camera {
        let theta = fov_y * std::f32::consts::PI / 180.0;
        let half_height = (theta * 0.5).tan();
        let half_width = aspect_ratio * half_height;

        let w = (position - look_at).normalize();
        let u = (up ^ w).normalize();
        let v = w ^ u;

        let top_left = position - (half_width * u) - (half_height * v) - w;
        let horizontal = 2.0 * half_width * u;
        let vertical = 2.0 * half_height * v;

        Camera { position: position,
            top_left: top_left,
            horizontal: horizontal,
            vertical: vertical,
            _u: u, _v: v, _w: w }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let dir = self.top_left
            + s * self.horizontal
            + (1.0 - t) * self.vertical
            - self.position;
        
        Ray::new(self.position, dir.normalize())
    }
}
