// #todo-refactor: Now this file is a spaghetti.

// ----------------------------------------------------------
// standard or 3rd party crates
use std::fs::File;
use std::sync::{Arc, Mutex};
use std::env;

use image::png::PngEncoder;
use image::ColorType;

use native_dialog::FileDialog;

use druid::widget::{Button, Flex, Label};
use druid::{Widget, WidgetExt};
use druid::{ExtEventSink, Selector, Command};

// ----------------------------------------------------------
// pvrlib package
use pvrlib::math::vec3::*;
use pvrlib::math::aabb::*;
use pvrlib::light::*;
use pvrlib::camera::*;
use pvrlib::scene::*;
use pvrlib::phasefn::*;
use pvrlib::voxelbuffer::dense::DenseBuffer;
use pvrlib::volume::voxel::*;
use pvrlib::volume::constant::*;
use pvrlib::volume::composite::*;
use pvrlib::primitive::*;
use pvrlib::primitive::rast::*;
use pvrlib::render::rendertarget::RenderTarget;
use pvrlib::render::renderer::*;

// ----------------------------------------------------------
// crate
use crate::gui::viewport::DruidViewport;
use crate::gui::settings::*;
use crate::timer::Stopwatch;
use crate::globalconstants::*;

// #todo-gui: Add scroll bar to the output log
const OUTPUT_LOG_MAX_LINES: usize = 20;

pub const START_RENDER_TASK: Selector<u32> = Selector::new("start_render_task");
pub const UPDATE_RENDER_PROGRESS: Selector<RenderProgressSelectorPayload> = Selector::new("update_render_progress");
pub const FINISH_RENDER_TASK: Selector<RenderTarget> = Selector::new("finish_render_task");

pub struct RenderProgressSelectorPayload {
    pub percent: u32,
    pub region: RenderRegion
}

pub struct RenderProgressWithDruid {
    total_pixels: u32,   // total pixels to render
    current_pixels: u32, // pixels rendered so far
    prev_percent: u32,
    event_sink: Option<druid::ExtEventSink>
}
impl RenderProgressWithDruid {
    pub fn new(event_sink: Option<druid::ExtEventSink>) -> Self {
        RenderProgressWithDruid {
            total_pixels: 0,
            current_pixels: 0,
            prev_percent: 0,
            event_sink: event_sink
        }
    }
}
impl RenderProgress for RenderProgressWithDruid {
    fn set_total(&mut self, total_pixels: u32) {
        self.total_pixels = total_pixels;
    }
    fn update(&mut self, subregion: &RenderRegion) {
        self.current_pixels += subregion.data.len() as u32;
        
        let ratio = (self.current_pixels as f32) / (self.total_pixels as f32);
        //let new_percent = 10 * ((10.0 * ratio) as u32);
        let new_percent = (100.0 * ratio) as u32;

        let should_send_command = true; // new_percent != self.prev_percent
        let should_log = new_percent != self.prev_percent;

        if should_send_command {
            self.prev_percent = new_percent;
            if let Some(_sink) = &self.event_sink {
                let payload = RenderProgressSelectorPayload {
                    percent: new_percent, 
                    region: subregion.clone()
                };
                _sink
                    .submit_command(UPDATE_RENDER_PROGRESS, payload, None)
                    .expect("Failed to submit: UPDATE_RENDER_PROGRESS");
            }
        }
        if should_log {
            println!("progress: {} %", new_percent);
        }
    }
}

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
    default_work_group_size: (usize, usize),
    default_exposure: f32,
    default_gamma_correction: f32,
    default_primary_step_size: f32,
    default_secondary_step_size: f32,
    pub work_group_size_x_input: String,
    pub work_group_size_y_input: String,
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
            default_work_group_size: render_settings.work_group_size,
            default_exposure: render_settings.exposure,
            default_gamma_correction: render_settings.gamma,
            default_primary_step_size: render_settings.primary_step_size,
            default_secondary_step_size: render_settings.secondary_step_size,
            work_group_size_x_input: render_settings.work_group_size.0.to_string(),
            work_group_size_y_input: render_settings.work_group_size.1.to_string(),
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
            work_group_size: self.default_work_group_size,
            exposure: self.default_exposure,
            gamma: self.default_gamma_correction,
            primary_step_size: self.default_primary_step_size,
            secondary_step_size: self.default_secondary_step_size,
        };

        if let Ok(work_group_size_x_parsed) = self.work_group_size_x_input.parse::<usize>() {
            settings.work_group_size.0 = work_group_size_x_parsed.max(4);
        }
        if let Ok(work_group_size_y_parsed) = self.work_group_size_y_input.parse::<usize>() {
            settings.work_group_size.1 = work_group_size_y_parsed.max(4);
        }
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

// #todo-gui: druid-shell complains about 'dropped message' like crazy while a dialog is open
fn browse_save_path() -> Option<std::path::PathBuf> {
    let cd = env::current_dir().unwrap();
    let query = FileDialog::new()
        .set_location(&cd)
        .add_filter("PNG image", &["png"])
        .show_save_single_file();
    
    let ret = match query {
        Err(e) => {
            println!("ERROR: Failed to open a file dialog: {}", e);
            None
        },
        Ok(None) => {
            println!("Save was cancelled");
            None
        }
        Ok(Some(x)) => Some(x)
    };

    ret
}

pub fn build_gui() -> impl Widget<AppState> {
    let viewport = DruidViewport::new(IMAGE_WIDTH, IMAGE_HEIGHT)
        .center();

    //let text = LocalizedString::new("hello-counter")
    //	.with_arg("count", |data: &AppState, _env| (*data).progress.into());
    //let label = Label::new(text)

    let label = Label::new(|data: &AppState, _env: &druid::Env| {
            format!("Progress: {}%", data.progress)
        })
        .padding(5.0)
        .center();
    let render_button = Button::new("Render")
        .on_click(|_ctx, data: &mut AppState, _env| {
            // Run async render job
            if data.can_launch_render_job() {
                let cmd = Command::new(START_RENDER_TASK, 0);
                _ctx.submit_command(cmd, None);
            } else {
                println!("ERROR: Renderer is already busy (caught in button widget)");
            }
        })
        .padding(5.0);
    let save_button = Button::new("Save as PNG")
        .on_click(|_ctx, data: &mut AppState, _env| {
            let buffer = data.render_result.lock().unwrap();
            if buffer.len() > 0 {
                if let Some(filename) = browse_save_path() {
                    let path = filename.to_str().unwrap();
                    print_rawbuffer(&buffer, IMAGE_WIDTH as u32, IMAGE_HEIGHT as u32, path);
                    println!("> Write the result to {}", path);
                }
            } else {
                println!("ERROR: No output has been generated");
            }
        })
        .padding(5.0);

    let mut col_render = Flex::column();
    col_render.add_flex_child(viewport, 1.0);
    col_render.add_child(label);
    col_render.add_child(render_button);
    col_render.add_child(save_button);

    /////////////////////////////////////////////////////////////////////
    // final hierarchy

    Flex::row()
        .with_flex_child(col_render, 0.5)
        .with_flex_child(build_ui_settings(), 0.2)
        .with_flex_spacer(0.1)
        .with_flex_child(build_ui_output_log(), 0.2)
}

fn print_rendertarget(rendertarget: &RenderTarget, filepath: &str) {
    let out_file = File::create(filepath).unwrap();
    let encoder = PngEncoder::new(&out_file);

    let buffer: Vec<u8> = rendertarget.generate_ldr_buffer();
    let width: u32 = rendertarget.get_width() as u32;
    let height: u32 = rendertarget.get_height() as u32;
    let color_type: ColorType = ColorType::Rgb8;

    encoder.encode(&buffer, width, height, color_type).unwrap();

    out_file.sync_all().unwrap();
}

fn print_rawbuffer(buffer: &Vec<u8>, width: u32, height: u32, filepath: &str) {
    let out_file = File::create(filepath).unwrap();
    let encoder = PngEncoder::new(&out_file);
    let color_type: ColorType = ColorType::Rgb8;
    encoder.encode(&buffer, width, height, color_type).unwrap();
    out_file.sync_all().unwrap();
}

// sink: Druid context for GUI update. (None if no gui mode)
pub fn begin_render(sink: Option<ExtEventSink>, render_settings: RenderSettings) {
    let aspect_ratio = (IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32);
    let mut rt: RenderTarget = RenderTarget::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    //test_sparse_buffer();

    let mut stopwatch = Stopwatch::new();

    // ----------------------------------------------------------
    // Modeling (#todo: move to modeler)
    
    println!("> Rasterizing primitives into voxel buffer...");

    stopwatch.start("rasterization");

    // #todo-emptyspace: Sparse buffer is 20x times slower
    //let voxel_buffer = SparseBuffer::new(
    let voxel_buffer = DenseBuffer::new(VOXEL_RESOLUTION);
    let mut voxel_volume = VoxelVolume {
        buffer: Box::new(voxel_buffer),
        //phase_fn: Box::new(HenyeyGreenstein{g: 0.76}),
        phase_fn: Box::new(DoubleHenyeyGreenstein{g1: 0.76, g2: -0.5, b: 0.2}),
        emission_value: vec3(0.0, 0.0, 0.0),
        absorption_coeff: vec3(0.75, 0.92, 0.72),
        world_bounds: AABB { min: vec3(-20.0, -20.0, -20.0), max: vec3(20.0, 20.0, 20.0) }
    };

    let point_prim = pyroclastic_point::PyroclasticPoint {
        center: vec3(0.0, 0.0, 0.0),
        radius: 8.0
    };
    point_prim.rasterize(&mut voxel_volume);

    // #todo-line: Raymarcher step size is too big, this does not look like a line
    let line_prim = line::Line {
        p0: vec3(-20.0, 10.0, 0.0),
        p1: vec3(20.0, 10.0, 0.0),
        radius: 1.0
    };
    line_prim.rasterize(&mut voxel_volume);

    println!("Buffer occupancy: {}", voxel_volume.get_buffer().get_occupancy());

    stopwatch.stop();

    let constant_volume = ConstantVolume::new(
        ConstantVolumeShape::Box,
        vec3(-8.0, -8.0, 0.0),  // center
        2.0,                    // radius
        vec3(0.1, 0.1, 0.1),    // emission
        vec3(0.86, 0.85, 0.95), // absorption coefficient
        vec3(1.0, 1.0, 1.0),    // scattering coefficient
        Box::new(Isotropic{})); // phaseFn

    let scene = Scene {
        volume: Box::new(CompositeVolume {
            children: vec![Box::new(voxel_volume), Box::new(constant_volume)]
        }),
        // #todo-light: These intensities are too big? Something wrong with lighting calculation?
        lights: vec![
            Box::new(PointLight {
                position: vec3(80.0, -20.0, 20.0),
                intensity: 5.0 * vec3(1.0, 1.0, 10000.0)
            }),
            Box::new(PointLight {
                position: vec3(-50.0, 20.0, -10.0),
                intensity: 5.0 * vec3(10000.0, 1.0, 1.0)
            })
        ]
    };

    // +x to right, +y to up, -z toward screen
    let camera = Camera::new(
        vec3(0.0, 0.0, 30.0), // origin
        vec3(0.0, 0.0, -1.0), // lookAt
        vec3(0.0, 1.0, 0.0),  // upVector
        FOV_Y, aspect_ratio);

    // ----------------------------------------------------------
    // Rendering
    println!("> Rendering the voxel buffer...");
    stopwatch.start("rendering");

    let sink_clone = match &sink {
        Some(_sink) => Some(_sink.clone()),
        None => None
    };
    let mut progress = Mutex::new(RenderProgressWithDruid::new(sink_clone));

    let mut renderer = Renderer::new(render_settings, &mut rt, &mut progress);
    renderer.render(&camera, &scene);

    stopwatch.stop();
    
    println!("> Write the result to {}", FILENAME);

    print_rendertarget(&rt, FILENAME);

    println!("Done.");
    
    match &sink {
        Some(_sink) => {
            _sink.submit_command(FINISH_RENDER_TASK, rt, None)
                .expect("Failed to submit: FINISH_RENDER_TARGET");
        },
        None => {
            //
        }
    }
}
