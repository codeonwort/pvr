use std::sync::{Arc, Mutex};

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
    pub dummy_gamma_string: String
}
