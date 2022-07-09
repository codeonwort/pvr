use std::sync::{Arc, Mutex};

use pvrlib::math::vec3::*;
use pvrlib::render::rendertarget::RenderTarget;
use pvrlib::render::renderer::RenderRegion;

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
    render_job_state: RenderJobState,
    pub progress: u32, // render job progress (0 ~ 100)
    pub render_result: Arc<Mutex<Vec<u8>>>,
    pub dummy_gamma_string: String,
    output_log: Arc<Mutex<Vec<String>>>,
    temp_render_target: Arc<Mutex<RenderTarget>>
}

impl AppState {
    pub fn new(render_image_size: (usize, usize), initial_gamma: f32) -> AppState {
        AppState {
            render_job_state: RenderJobState::IDLE,
            progress: 0,
            render_result: Arc::new(Mutex::new(Vec::new())),
            // #todo-gui: What on earth is this?
            dummy_gamma_string: initial_gamma.to_string(),
            output_log: Arc::new(Mutex::new(vec!["=== Output Log ===".to_string()])),
            temp_render_target: Arc::new(Mutex::new(RenderTarget::new(render_image_size.0, render_image_size.1)))
        }
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
