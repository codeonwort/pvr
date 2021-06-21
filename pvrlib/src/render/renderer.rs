use super::rendertarget::*;
use super::raymarcher::*;
use crate::math::vec3::*;
use crate::camera::Camera;
use crate::scene::Scene;

use std::ops::Deref;
use std::sync::*;

use rayon::prelude::*;

pub struct RenderSettings {
    pub exposure: f32,
    pub gamma: f32
}

pub trait RenderProgress : Send {
    fn set_total(&mut self, total: u32);
    fn update(&mut self, append: u32);
}

pub struct Renderer<'a> {
    settings: RenderSettings,
    render_target: &'a mut RenderTarget,
    // #todo: Still uncomfortable with this borrow-shit... Is it a proper way?
    progress: &'a mut Mutex<dyn RenderProgress>
}

struct RenderRegion {
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
    pub data: Vec<Vec3>
}

impl Renderer<'_> {

    pub fn new<'a>(settings: RenderSettings, render_target: &'a mut RenderTarget, progress: &'a mut Mutex<dyn RenderProgress>) -> Renderer<'a> {
        Renderer { settings: settings, render_target: render_target, progress: progress }
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        let width = self.render_target.get_width();
        let height = self.render_target.get_height();
        let inv_width = 1.0 / (width as f32);
        let inv_height = 1.0 / (height as f32);

        // Partition the whole region into subregions
        let work_size = (32, 32);
        let work_group_size = (
            (width / 32) + if width % 32 == 0 { 0 } else { 1 },
            (height / 32) + if height % 32 == 0 { 0 } else { 1 }
        );
        let mut regions = Vec::new();
        for i in 0..(work_group_size.0) {
            for j in 0..(work_group_size.1) {
                let x = i * work_size.0;
                let y = j * work_size.1;
                let region = RenderRegion {
                    x0: x,
                    y0: y,
                    x1: std::cmp::min(x + work_size.0, width),
                    y1: std::cmp::min(y + work_size.1, height),
                    data: Vec::new()
                };
                regions.push(region);
            }
        }

        let exposure = self.settings.exposure;
        let gamma = self.settings.gamma;

        // Raymarching
        let total_pixels = width * height;
        self.progress.lock().unwrap().set_total(total_pixels as u32);
        
        regions.par_iter_mut().for_each(|r| {
            // Render a subregion
            for y in r.y0 .. r.y1 {
                for x in r.x0 .. r.x1 {
                    let u = (x as f32) * inv_width;
                    let v = (y as f32) * inv_height;
                    let ray = camera.get_ray(u, v);

                    let result = integrate_ray(scene.volume.deref(), ray, &scene.lights);
                    let mut luminance = result.luminance;
                    //let transmittance = result.transmittance;
                    
                    // tone mapping
                    luminance = vec3(1.0, 1.0, 1.0) - (-luminance * exposure).exp();
        
                    // gamma correction
                    luminance = luminance.pow(1.0 / gamma);
                    
                    // WTF Rust :(
                    // I can't directly modify self.render_target here?
                    r.data.push(luminance);
                }
            }
            // Update overall progress
            self.progress.lock().unwrap().update(((r.x1 - r.x0) * (r.y1 - r.y0)) as u32);
        });

        // Copy subregions to the final render target (This is really unncessary work...)
        regions.iter().for_each(|r| {
            let mut p = 0;
            for y in r.y0 .. r.y1 {
                for x in r.x0 .. r.x1 {
                    self.render_target.set(x as i32, y as i32, r.data[p]);
                    p += 1;
                }
            }
        });
    }

}
