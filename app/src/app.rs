use std::sync::{Arc, Mutex};

use pvrlib::math::vec3::*;
use pvrlib::render::rendertarget::RenderTarget;
use pvrlib::render::renderer::RenderRegion;
use pvrlib::render::renderer::RenderSettings;

// #todo-gui: Add scroll bar to the output log
const OUTPUT_LOG_MAX_LINES: usize = 20;

#[derive(Copy, Clone, PartialEq, druid::Data)]
pub enum RenderJobState {
    IDLE,
    BUSY,
    FINISHED
}

#[derive(Clone, druid::Data, druid::Lens)]
pub struct AppState {
    // Rendering status
    render_job_state: RenderJobState,
    pub progress: u32, // render job progress (0 ~ 100)
    pub render_result: Arc<Mutex<Vec<u8>>>,
    temp_render_target: Arc<Mutex<RenderTarget>>,
    // Render settings
    // #todo-druid: Can't impl or derive druid::Data for RenderSettings :/
    //              Let's copy each field manually...
    //default_render_settings: RenderSettings,
    default_exposure: f32,
    default_gamma_correction: f32,
    default_primary_step_size: f32,
    default_secondary_step_size: f32,
    pub exposure_input: String,
    pub gamma_correction_input: String,
    pub primary_step_size_input: String,
    pub secondary_step_size_input: String,
    // Misc
    output_log: Arc<Mutex<Vec<String>>>,
}

impl AppState {
    pub fn new(
        render_image_size: (usize, usize), render_settings: RenderSettings) -> AppState {
        let rt = RenderTarget::new(render_image_size.0, render_image_size.1);
        let logs = vec!["=== Output Log ===".to_string()];

        AppState {
            // Rendering status
            render_job_state: RenderJobState::IDLE,
            progress: 0,
            render_result: Arc::new(Mutex::new(Vec::new())),
            temp_render_target: Arc::new(Mutex::new(rt)),
            // Render settings
            default_exposure: render_settings.exposure,
            default_gamma_correction: render_settings.gamma,
            default_primary_step_size: render_settings.primary_step_size,
            default_secondary_step_size: render_settings.secondary_step_size,
            exposure_input: render_settings.exposure.to_string(),
            gamma_correction_input: render_settings.gamma.to_string(),
            primary_step_size_input: render_settings.primary_step_size.to_string(),
            secondary_step_size_input: render_settings.secondary_step_size.to_string(),
            // Misc
            output_log: Arc::new(Mutex::new(logs)),
        }
    }

    pub fn get_render_settings(&self) -> RenderSettings {
        let mut settings = RenderSettings {
            exposure: self.default_exposure,
            gamma: self.default_gamma_correction,
            primary_step_size: self.default_primary_step_size,
            secondary_step_size: self.default_secondary_step_size,
        };

        if let Ok(exposure_parsed) = self.exposure_input.parse::<f32>() {
            settings.exposure = exposure_parsed.max(0.01);
        }
        if let Ok(gamma_parsed) = self.gamma_correction_input.parse::<f32>() {
            // Values below 1.0 makes no sense, but just keep it positive at the minimum.
            settings.gamma = gamma_parsed.max(0.01);
        }
        if let Ok(stepsize1_parsed) = self.primary_step_size_input.parse::<f32>() {
            settings.primary_step_size = stepsize1_parsed.max(0.1);
        }
        if let Ok(stepsize2_parsed) = self.secondary_step_size_input.parse::<f32>() {
            settings.secondary_step_size = stepsize2_parsed.max(0.1);
        }

        settings
    }

    pub fn can_launch_render_job(&self) -> bool {
        self.render_job_state == RenderJobState::IDLE
        || self.render_job_state == RenderJobState::FINISHED
    }

    // Initialize fields before beginning of rendering.
    pub fn mark_begin_rendering(&mut self) {
        assert!(self.can_launch_render_job());
        self.render_job_state = RenderJobState::BUSY;
        self.progress = 0;
        self.temp_render_target.lock().unwrap().clear_color(vec3(0.0, 1.0, 0.0));
    }

    pub fn mark_finish_rendering(&mut self) {
        self.render_job_state = RenderJobState::FINISHED;
    }

    pub fn add_log(&mut self, log: &str) {
        let mut buf = self.output_log.lock().unwrap();
        buf.push(format!("{}", log));
        // #todo: Incredibly dumb splicing in my life :/
        // +1 for first line ("=== Output Log ===")
        if buf.len() > (1 + OUTPUT_LOG_MAX_LINES) {
            let del_count = buf.len() - OUTPUT_LOG_MAX_LINES - 1;
            let mut v = vec![buf[0].clone()];
            v.extend_from_slice(&buf[(1+del_count)..]);
            buf.clear();
            buf.append(&mut v);
        }
    }

    // Update intermediate rendering result.
    pub fn update_temp_image(&mut self, region: &RenderRegion) {
        self.temp_render_target.lock().unwrap().update_region(region);
    }
    pub fn generate_temp_image_buffer(&self) -> Vec<u8> {
        self.temp_render_target.lock().unwrap().generate_ldr_buffer()
    }

    pub fn get_all_log(&self) -> String {
        self.output_log.lock().unwrap().join("\n")
    }
}
