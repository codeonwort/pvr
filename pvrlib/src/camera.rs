use crate::math::vec3::Vec3;
use crate::math::ray::Ray;

pub struct Camera {
    position: Vec3,
    // precalculation
    top_left: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    // basis
    _u: Vec3,
    _v: Vec3,
    _w: Vec3,
}

impl Camera {
    pub fn new(position: Vec3, look_at: Vec3, up: Vec3, fov_y: f32, aspect_ratio: f32) -> Camera {
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
