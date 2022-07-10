use std::thread;

use druid::{AppDelegate, DelegateCtx, ExtEventSink, Target, Command, Env};

use crate::app::{AppState, begin_render};
use crate::app::{START_RENDER_TASK, UPDATE_RENDER_PROGRESS, FINISH_RENDER_TASK};

pub struct PVRAppDelegate {
    pub event_sink: ExtEventSink
}

impl AppDelegate<AppState> for PVRAppDelegate {
    fn command(
        &mut self,
        _ctx: &mut DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env
    ) -> bool {
        if cmd.is(START_RENDER_TASK) {
            if data.can_launch_render_job() {
                data.mark_begin_rendering();
                let event_sink_clone = self.event_sink.clone();
                let render_settings = data.get_render_settings();
                thread::spawn(move || {
                    begin_render(Some(event_sink_clone), render_settings);
                });
                data.add_log("Begin rendering...");
            } else {
                println!("Renderer is already busy (caught in the delegate)");
                data.add_log("FAILED: Renderer is already busy");
            }
        }
        if let Some(payload) = cmd.get(UPDATE_RENDER_PROGRESS) {
            let should_add_log = (payload.percent > 0) && (payload.percent > data.render_progress);
            data.render_progress = payload.percent;
            data.update_temp_image(&payload.region);
            if should_add_log {
                data.add_log(&format!("Progress: {} %", data.render_progress));
            }
        }
        if let Some(render_result) = cmd.get(FINISH_RENDER_TASK) {
            let mut ex_buffer = data.render_result.lock().unwrap();
            render_result.copy_to(&mut ex_buffer);
            drop(ex_buffer);
            data.mark_finish_rendering();
            data.add_log("Finish rendering...");
        }

        true
    }
}
