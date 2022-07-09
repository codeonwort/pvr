// ----------------------------------------------------------
// standard or 3rd party crates
use std::env;

use druid::{AppLauncher, WindowDesc};

// ----------------------------------------------------------
// crate
mod gui;
mod app;
mod appdelegate;
mod globalconstants;
mod timer;

use crate::app::*;
use crate::appdelegate::*;
use crate::globalconstants::*;

use pvrlib::render::renderer::*;

// ----------------------------------------------------------
// program code

fn main() {
    // Parse env arguments
    for arg in env::args() {
        if arg == "-nogui" {
            let event_sink = None;
            let settings = get_default_render_settings();
            begin_render(event_sink, settings);
            return;
        }
    }

    // Create and launch app.
    let main_window = WindowDesc::new(build_gui)
        .title(WINDOW_TITLE)
        .window_size((WINDOW_WIDTH, WINDOW_HEIGHT));

    let app = AppLauncher::with_window(main_window);
    let app_state = AppState::new((IMAGE_WIDTH, IMAGE_HEIGHT), get_default_render_settings());
    let delegate = PVRAppDelegate { event_sink: app.get_external_handle() };

    app.delegate(delegate)
        .use_simple_logger()
        .launch(app_state)
        .expect("Failed to launch app");
}

fn get_default_render_settings() -> RenderSettings {
    RenderSettings {
        exposure: EXPOSURE,
        gamma: GAMMA_VALUE,
        primary_step_size: STEP_SIZE_1ST,
        secondary_step_size: STEP_SIZE_2ND,
    }
}
