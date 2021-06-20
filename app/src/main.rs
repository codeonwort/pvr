// ----------------------------------------------------------
// standard or 3rd party crates
use image::png::PngEncoder;
use image::ColorType;
use std::fs::File;
use std::thread;
use std::sync::{Arc, Mutex};
use std::env;

use druid::widget::{Button, Flex, Label};
use druid::{AppLauncher, LocalizedString, PlatformError, Widget, WidgetExt, WindowDesc};
use druid::{AppDelegate, DelegateCtx, ExtEventSink, Selector, Target, Command, Env};

// ----------------------------------------------------------
// pvrlib package
use pvrlib::math::vec3::*;
use pvrlib::math::aabb::*;

// ----------------------------------------------------------
// module: gui
mod gui;
use gui::viewport::DruidViewport;

// ----------------------------------------------------------
// module: volume
mod volume;
use volume::voxel::*;
use volume::constant::ConstantVolume;
use volume::composite::CompositeVolume;

// ----------------------------------------------------------
// module: voxel, primitive
mod voxel;
mod noise;
mod primitive;
use voxel::voxel::VoxelBuffer;
use voxel::dense::DenseBuffer;
use voxel::sparse::SparseBuffer;
use primitive::primitive::*;
use noise::*;

// ----------------------------------------------------------
// module: rendertarget
mod rendertarget;
use rendertarget::RenderTarget;

// ----------------------------------------------------------
// module: camera
mod camera;
use camera::Camera;

// ----------------------------------------------------------
// module: raymarcher
mod raymarcher;

// ----------------------------------------------------------
// module: light
mod light;
use light::*;

// ----------------------------------------------------------
// module: scene, renderer
mod scene;
mod renderer;
use scene::*;
use renderer::*;

// ----------------------------------------------------------
mod timer;
use timer::Stopwatch;

// ----------------------------------------------------------
// program code
const WINDOW_TITLE: &str = "PVR GUI";
const WINDOW_WIDTH: f64 = 1280.0;
const WINDOW_HEIGHT: f64 = 768.0;

const IMAGE_WIDTH: usize = 512;
const IMAGE_HEIGHT: usize = 512;
const FILENAME: &str = "test.png";

const GAMMA_VALUE: f32 = 2.2;
const FOV_Y: f32 = 45.0;
const EXPOSURE: f32 = 1.2;
const VOXEL_RESOLUTION: (i32, i32, i32) = (512, 512, 256);

#[derive(Copy, Clone, PartialEq, druid::Data)]
pub enum RenderJobState {
	IDLE,
	BUSY,
	FINISHED
}

#[derive(Clone, druid::Data)]
pub struct AppState {
	pub render_job_state: RenderJobState,
	pub progress: u32, // render job progress (0 ~ 100)
	pub render_result: Arc<Mutex<Vec<u8>>>
}

fn can_launch_render_job(current_state: RenderJobState) -> bool {
	current_state == RenderJobState::IDLE
	|| current_state == RenderJobState::FINISHED
}

pub const START_RENDER_TASK: Selector<u32> = Selector::new("start_render_task");
pub const UPDATE_RENDER_PROGRESS: Selector<u32> = Selector::new("update_render_progress");
pub const FINISH_RENDER_TASK: Selector<RenderTarget> = Selector::new("finish_render_task");

struct Delegate {
	event_sink: ExtEventSink
}

impl AppDelegate<AppState> for Delegate {
	fn command(
		&mut self,
		_ctx: &mut DelegateCtx,
		_target: Target,
		cmd: &Command,
		data: &mut AppState,
		_env: &Env
	) -> bool {
		if cmd.is(START_RENDER_TASK) {
			if can_launch_render_job(data.render_job_state) {
				data.render_job_state = RenderJobState::BUSY;
				let event_sink_clone = self.event_sink.clone();
				thread::spawn(move || {
					begin_render(Some(event_sink_clone));
				});
			} else {
				println!("Renderer is already busy (caught in the delegate)");
			}
		}
		if let Some(progress) = cmd.get(UPDATE_RENDER_PROGRESS) {
			data.progress = *progress;
		}
		if let Some(render_result) = cmd.get(FINISH_RENDER_TASK) {
			let mut ex_buffer = data.render_result.lock().unwrap();
			render_result.copy_to(&mut ex_buffer);
			data.render_job_state = RenderJobState::FINISHED;
		}

		true
	}
}

fn main() {
	for arg in env::args() {
		if arg == "-nogui" {
			begin_render(None);
			return;
		}
	}

	let main_window = WindowDesc::new(ui_builder)
		.title(WINDOW_TITLE)
		.window_size((WINDOW_WIDTH, WINDOW_HEIGHT));

	let app = AppLauncher::with_window(main_window);

	let app_state = AppState {
		render_job_state: RenderJobState::IDLE,
		progress: 0,
		render_result: Arc::new(Mutex::new(Vec::new()))
	};

	let delegate = Delegate {
		event_sink: app.get_external_handle()
	};

	app.delegate(delegate)
		.use_simple_logger()
		.launch(app_state)
		.expect("Failed to launch app");
}

fn ui_builder() -> impl Widget<AppState> {
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
			if can_launch_render_job(data.render_job_state) {
				let cmd = Command::new(START_RENDER_TASK, 0);
				_ctx.submit_command(cmd, None);
			} else {
				println!("Renderer is already busy (caught in button widget)");
			}
		})
		.padding(5.0);
	// #todo-gui: File browser (seems browse API is absent in std)
	//let save_button = Button::new("Save as PNG (wip)")
	//	.on_click(|_ctx, data: &mut AppState, _env| { /* todo */ })
	//	.padding(5.0);

	let mut col = Flex::column();
	col.add_flex_child(viewport, 1.0);
	col.add_child(label);
	col.add_child(render_button);
	//col.add_child(save_button);

	col
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

fn noise_test(rt: &mut RenderTarget) {
	let width = rt.get_width();
	let height = rt.get_height();
	let inv_width = 1.0 / (width as f32);
	let inv_height = 1.0 / (height as f32);

	let z: f32 = 0.0;
	for y in 0..height {
        for x in 0..width {
			let u = 2.0 * (x as f32) * inv_width - 1.0;
			let v = 2.0 * (y as f32) * inv_height - 1.0;
			let uv_len = (1.0_f32 - z * z).sqrt();
			let p = vec3(uv_len * u, uv_len * v, z);
			
			let noise = fBm(p * 4.0);

			let sphere_func = p.length() - 1.0;
			let filter_width = 2.0;
			let pyro = pyroclastic(sphere_func, noise, filter_width);

			rt.set(x as i32, y as i32, pyro.into());
		}
	}
}

fn test_sparse_buffer() {
	println!("=== TEST SPARSE BUFFER ===");
	let bounds = AABB { min: vec3(-20.0, -20.0, -20.0), max: vec3(20.0, 20.0, 20.0) };
	let mut buffer = SparseBuffer::new((512, 512, 256), bounds);

	println!("> write sparse buffer...");
	buffer.write(0, 0, 0, vec3(3.0, 4.0, 5.0));
	buffer.write(50, 0, 70, vec3(7.0, 5.0, 2.0));
	buffer.write(5, 0, 99, vec3(8.0, 1.0, 6.0));
	buffer.write(99, 99, 99, vec3(5.0, 3.0, 1.0));
	buffer.write(46, 0, -270, vec3(31.0, 42.0, 53.0));
	//for y in 0..512 { buffer.write(0, y, 0, vec3(y as f32, 1.0, 1.0)); }

	println!("> read sparse buffer...");
	println!("buffer[0,0,0] = {:?}", buffer.read(0, 0, 0));
	println!("buffer[50,0,70] = {:?}", buffer.read(50, 0, 70));
	println!("buffer[5,0,99] = {:?}", buffer.read(5, 0, 99));
	println!("buffer[99,99,99] = {:?}", buffer.read(99, 99, 99));
	println!("buffer[46,0,-270] = {:?}", buffer.read(46, 0, -270));
	//for y in 0..512 { println!("buffer[0,{},0] = {:?}", y, buffer.read(0, y, 0)); }

	/*
	// occupancy debugging
	{
		let buffer_size = (64, 64, 64);
		let mut buffer = SparseBuffer::new(buffer_size, bounds);
		println!("> buffer size: {:?}", buffer_size);

		println!("> occupancy = {}", buffer.get_occupancy());

		println!("> write to (0,0,0)");
		buffer.write(0, 0, 0, vec3(1.0, 1.0, 1.0));
		println!("> occupancy = {}", buffer.get_occupancy());

		println!("> write to (63,63,63)");
		buffer.write(63, 63, 63, vec3(1.0, 1.0, 1.0));
		println!("> occupancy = {}", buffer.get_occupancy());
	}
	*/

	println!("=== END TEST SPARSE BUFFER ===");
}

fn begin_render(sink: Option<ExtEventSink>) {
	let aspect_ratio = (IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32);
	let mut rt: RenderTarget = RenderTarget::new(IMAGE_WIDTH, IMAGE_HEIGHT);

	//test_sparse_buffer();

	let mut stopwatch = Stopwatch::new();

	// ----------------------------------------------------------
	// Modeling (#todo: move to modeler)
	
	println!("> Rasterizing primitives into voxel buffer...");

	stopwatch.start("rasterization");

	let voxel_buffer = DenseBuffer::new(
		VOXEL_RESOLUTION,
		AABB { min: vec3(-20.0, -20.0, -20.0), max: vec3(20.0, 20.0, 20.0) });
	let mut voxel_volume = VoxelVolume {
		buffer: Box::new(voxel_buffer),
		emission_value: vec3(0.0, 0.0, 0.0),
		absorption_coeff: vec3(0.75, 0.92, 0.72)
	};

	let point_prim = primitive::rast::pyroclastic_point::PyroclasticPoint {
		center: vec3(0.0, 0.0, 0.0),
		radius: 8.0
	};
	point_prim.rasterize(voxel_volume.get_buffer());

	let line_prim = primitive::rast::line::Line {
		p0: vec3(-20.0, 10.0, 0.0),
		p1: vec3(20.0, 10.0, 0.0),
		radius: 4.0
	};
	line_prim.rasterize(voxel_volume.get_buffer());

	println!("Buffer occupancy: {}", voxel_volume.get_buffer().get_occupancy());

	stopwatch.stop();

	let constant_volume = ConstantVolume::new(
		vec3(-10.0, 0.0, 0.0), 2.0, vec3(0.02, 0.02, 0.02), vec3(0.76, 0.65, 0.95));

	let scene = Scene {
		volume: Box::new(CompositeVolume {
			children: vec![Box::new(voxel_volume), Box::new(constant_volume)]
		}),
		lights: vec![
			Box::new(PointLight {
				position: vec3(80.0, -20.0, 20.0),
				intensity: 5.0 * vec3(0.0, 0.0, 10000.0)
			}),
			Box::new(PointLight {
				position: vec3(-50.0, 20.0, -10.0),
				intensity: 5.0 * vec3(10000.0, 0.0, 0.0)
			})
		]
	};

	let camera = Camera::new(
		vec3(0.0, 0.0, 30.0), vec3(0.0, 0.0, -1.0), vec3(0.0, 1.0, 0.0),
		FOV_Y, aspect_ratio);

	// ----------------------------------------------------------
	// Rendering
	println!("> Rendering the voxel buffer...");
	stopwatch.start("rendering");

	let render_settings = RenderSettings {
		exposure: EXPOSURE,
		gamma: GAMMA_VALUE
	};
	let mut renderer = Renderer::new(render_settings, &mut rt);

	let sink_clone = match &sink {
		Some(_sink) => Some(_sink.clone()),
		None => None
	};
	renderer.render(sink_clone, &camera, &scene);

	stopwatch.stop();

	// Comment out rasterization and rendering to test noise
	//noise_test(&mut rt);
	
	println!("> Printing the image to {}", FILENAME);

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