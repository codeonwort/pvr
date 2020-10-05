// ----------------------------------------------------------
// standard or 3rd party crates
use image::png::PngEncoder;
use image::ColorType;
use std::fs::File;

// ----------------------------------------------------------
// (math) module: vec3, aabb
mod aabb;
mod vec3;
use vec3::*;
use aabb::*;

// ----------------------------------------------------------
// module: voxel, primitive and volume
mod voxel;
mod volume;
mod noise;
mod primitive;
use voxel::VoxelBuffer;
use volume::constant_volume::ConstantVolume;
use volume::voxel_volume::*;
use primitive::primitive::Primitive;
use noise::*;

// ----------------------------------------------------------
// module: rendertarget
mod rendertarget;
use rendertarget::RenderTarget;

// ----------------------------------------------------------
// module: ray, and camera
mod ray;
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
// program code
const IMAGE_WIDTH: usize = 512;
const IMAGE_HEIGHT: usize = 512;
const FILENAME: &str = "test.png";

const GAMMA_VALUE: f32 = 2.2;
const FOV_Y: f32 = 45.0;
const EXPOSURE: f32 = 1.2;
const VOXEL_RESOLUTION: (i32, i32, i32) = (512, 512, 256);

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

fn main() {
	let aspect_ratio = (IMAGE_WIDTH as f32) / (IMAGE_HEIGHT as f32);
	let mut rt: RenderTarget = RenderTarget::new(IMAGE_WIDTH, IMAGE_HEIGHT);

	// ----------------------------------------------------------
	// Modeling (#todo: move to modeler)
	println!("> Rasterizing primitives into voxel buffer...");

	let voxel_buffer = VoxelBuffer::new(
		VOXEL_RESOLUTION,
		AABB { min: vec3(-20.0, -20.0, -20.0), max: vec3(20.0, 20.0, 20.0) });
	let mut voxel_volume = VoxelVolume {
		buffer: voxel_buffer,
		emission_value: vec3(0.0, 0.0, 0.0),
		absorption_coeff: vec3(0.75, 0.92, 0.72)
	};
	let point_prim = primitive::pyroclastic_point::PyroclasticPoint {
		center: vec3(0.0, 0.0, 0.0),
		radius: 12.0
	};
	point_prim.rasterize(voxel_volume.get_buffer());

	// Test scene (#todo: CompositeVolume)
	let constant_volume = ConstantVolume::new(
		vec3(0.0, 0.0, 0.0), 8.0, vec3(0.8, 0.1, 0.2), vec3(0.76, 0.65, 0.95));
	let scene = Scene {
		volume: Box::new(voxel_volume),
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

	let render_settings = RenderSettings {
		exposure: EXPOSURE,
		gamma: GAMMA_VALUE
	};
	let mut renderer = Renderer::new(render_settings, &mut rt);
	renderer.render(&camera, &scene);

	// Comment out rasterization and rendering to test noise
	//noise_test(&mut rt);
	
	println!("> Printing the image to {}", FILENAME);

	print_rendertarget(&rt, FILENAME);

    println!("Done.");
}
