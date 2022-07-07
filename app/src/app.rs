use std::sync::{Arc, Mutex};

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
    pub render_job_state: RenderJobState,
    pub progress: u32, // render job progress (0 ~ 100)
    pub render_result: Arc<Mutex<Vec<u8>>>,
    pub dummy_gamma_string: String,
    output_log: Arc<Mutex<Vec<String>>>
}

impl AppState {
    pub fn new(initial_gamma: f32) -> AppState {
        AppState {
            render_job_state: RenderJobState::IDLE,
            progress: 0,
            render_result: Arc::new(Mutex::new(Vec::new())),
            // #todo-gui: What on earth is this?
            dummy_gamma_string: initial_gamma.to_string(),
            output_log: Arc::new(Mutex::new(vec!["=== Output Log ===".to_string()]))
        }
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

    pub fn get_all_log(&self) -> String {
        self.output_log.lock().unwrap().join("\n")
    }
}
