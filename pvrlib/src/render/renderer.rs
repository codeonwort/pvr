use super::rendertarget::*;
use super::raymarcher::*;
use crate::math::vec3::*;
use crate::camera::Camera;
use crate::scene::Scene;
use crate::skyatmosphere::SkyAtmosphere;
use crate::render::tone_mapping::*;

use std::ops::Deref;
use std::sync::*;

use rayon::prelude::*;

// Options to setup before starting the rendering.
#[derive(Copy, Clone)]
pub struct RenderSettings {
    pub work_group_size: (usize, usize),
    pub exposure: f32,
    pub gamma: f32,
    pub primary_step_size: f32,
    pub secondary_step_size: f32,
    pub draw_sky: bool,
    pub camera_origin: vec3f,
    pub camera_lookat: vec3f,
    pub fov: f32,
}

// Handles change of render progress.
// ex) RenderProgressWithDruid updates a progress bar widget.
pub trait RenderProgress : Send {
    fn set_total(&mut self, total_pixels: u32);
    fn update(&mut self, subregion: &RenderRegion);
}

pub struct Renderer<'a> {
    settings: RenderSettings,
    render_target: &'a mut RenderTarget,
    // #todo: Still uncomfortable with this borrow-shit... Is it a proper way?
    progress: &'a mut Mutex<dyn RenderProgress>
}

#[derive(Clone)]
pub struct RenderRegion {
    pub x0: usize,
    pub y0: usize,
    pub x1: usize,
    pub y1: usize,
    pub data: Vec<vec3f>
}

impl Renderer<'_> {

    pub fn new<'a>(
        settings: RenderSettings,
        render_target: &'a mut RenderTarget,
        progress: &'a mut Mutex<dyn RenderProgress>) -> Renderer<'a>
    {
        Renderer {
            settings: settings,
            render_target: render_target,
            progress: progress
        }
    }

    pub fn render(&mut self, camera: &Camera, scene: &Scene) {
        let width = self.render_target.get_width();
        let height = self.render_target.get_height();
        let inv_width = 1.0 / (width as f32);
        let inv_height = 1.0 / (height as f32);

        // Partition the whole region into subregions
        let work_group_size = self.settings.work_group_size;
        let work_group_count = (
            (width / work_group_size.0) + if width % work_group_size.0 == 0 { 0 } else { 1 },
            (height / work_group_size.1) + if height % work_group_size.1 == 0 { 0 } else { 1 }
        );
        let mut regions = Vec::new();
        for i in 0..(work_group_count.0) {
            for j in 0..(work_group_count.1) {
                let x = i * work_group_size.0;
                let y = j * work_group_size.1;
                let region = RenderRegion {
                    x0: x,
                    y0: y,
                    x1: std::cmp::min(x + work_group_size.0, width),
                    y1: std::cmp::min(y + work_group_size.1, height),
                    data: Vec::new()
                };
                regions.push(region);
            }
        }

        let exposure = self.settings.exposure;
        let gamma = self.settings.gamma;
        let step_size1 = self.settings.primary_step_size;
        let step_size2 = self.settings.secondary_step_size;

        // Raymarching
        let total_pixels = width * height;
        self.progress.lock().unwrap().set_total(total_pixels as u32);
        
        regions.par_iter_mut().for_each(|region| {
            // Render a subregion
            for y in region.y0 .. region.y1 {
                for x in region.x0 .. region.x1 {
                    let u = (x as f32) * inv_width;
                    let v = (y as f32) * inv_height;
                    let ray = camera.get_ray(u, v);

                    let result: IntegrationResult = integrate_ray(
                        scene.volume.deref(),
                        ray,
                        &scene.lights,
                        step_size1,
                        step_size2);
                    
                    let mut luminance = result.luminance;
                    let transmittance = result.transmittance;
                    
                    // #todo-sky: Move into integrate_ray?
                    // Atmosphere is not a mere background texture. It should affect volumes on the ground.
                    let ray_on_earth = SkyAtmosphere::get_camera_ray_on_earth(ray);
                    let sky_sample = scene.sky_atmosphere.sample(ray_on_earth);
                    luminance += sky_sample * transmittance;

                    // Tone mapping and gamma correction
                    luminance = aces_tone_mapping(luminance * exposure);
                    luminance = luminance.pow(1.0 / gamma);
                    
                    // WTF Rust :(
                    // I can't directly modify self.render_target here?
                    region.data.push(luminance);
                }
            }
            // Update overall progress
            self.progress.lock().unwrap().update(region);
        });

        // Copy subregions to the final render target (This is really unncessary work...)
        regions.iter().for_each(|region| {
            self.render_target.update_region(region);
        });
    }

}
