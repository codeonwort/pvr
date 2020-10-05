use crate::vec3::*;
use crate::rendertarget::*;
use crate::raymarcher::*;
use crate::camera::*;
use crate::volume::volume::*;
use crate::light::*;

pub struct RenderSettings {
    pub exposure: f32,
    pub gamma: f32
}

pub struct Renderer<'a> {
    settings: RenderSettings,
    render_target: &'a mut RenderTarget
}

impl Renderer<'_> {

    pub fn new(settings: RenderSettings, render_target: &mut RenderTarget) -> Renderer {
        Renderer { settings: settings, render_target: render_target }
    }

    pub fn render(&mut self, camera: &Camera, volume: &dyn Volume, lights: &[Box<dyn Light>]) {
        let width = self.render_target.get_width();
        let height = self.render_target.get_height();
        let inv_width = 1.0 / (width as f32);
        let inv_height = 1.0 / (height as f32);

        let mut progress = 0;
        let mut progress_prev = 0;

        for y in 0..height {
            for x in 0..width {
                let u = (x as f32) * inv_width;
                let v = (y as f32) * inv_height;
                let ray = camera.get_ray(u, v);
    
                let result = integrate_ray(volume, ray, &lights);
                let mut luminance = result.luminance;
                //let transmittance = result.transmittance;
                
                // tone mapping
                luminance = vec3(1.0, 1.0, 1.0) - (-luminance * self.settings.exposure).exp();
    
                // gamma correction
                luminance = luminance.pow(1.0 / self.settings.gamma);
    
                self.render_target.set(x as i32, y as i32, luminance);
            }
            
            progress = (10.0 * (y as f32) / (height as f32)) as i32;
            if progress != progress_prev {
                println!("progress: {} %", progress * 10);
                progress_prev = progress;
            }
        }
    }

}
