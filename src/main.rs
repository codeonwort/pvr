// ----------------------------------------------------------
// standard or 3rd party crates
use image::png::PNGEncoder;
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
use volume::ConstantVolume;
use primitive::primitive::Primitive;
use primitive::point;

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
use raymarcher::*;

// ----------------------------------------------------------
// module: light
mod light;
use light::*;

// ----------------------------------------------------------
// program code
const FILENAME: &str = "test.png";
const GAMMA_VALUE: f32 = 2.2;
const FOV_Y: f32 = 45.0;
const EXPOSURE: f32 = 1.2;

fn print_rendertarget(rendertarget: &RenderTarget, filepath: &str) {
	let out_file = File::create(filepath).unwrap();
	let encoder = PNGEncoder::new(&out_file);

	let buffer: Vec<u8> = rendertarget.generate_ldr_buffer();
	let width: u32 = rendertarget.get_width() as u32;
	let height: u32 = rendertarget.get_height() as u32;
	let color_type: ColorType = ColorType::Rgb8;

	encoder.encode(&buffer, width, height, color_type).unwrap();

    out_file.sync_all().unwrap();
}

fn main() {
	// ----------------------------------------------------------
	// Environments (#todo: make configurable)
    let width: usize = 512;
	let height: usize = 512;
	let aspect_ratio = (width as f32) / (height as f32);
	let mut rt: RenderTarget = RenderTarget::new(width, height);

	// ----------------------------------------------------------
	// Modeling (#todo: move to modeler)
	let mut voxel_buffer = VoxelBuffer::new(
		(128, 128, 128),
		AABB { min: vec3(-20.0, -20.0, -20.0), max: vec3(20.0, 20.0, 20.0) });
	let point_prim = primitive::point::Point { center: vec3(0.0, 0.0, 0.0), radius: 5.0 };
	point_prim.rasterize(&mut voxel_buffer);

	let camera = Camera::new(
		vec3(0.0, 0.0, -10.0), vec3(0.0, 0.0, 10.0), vec3(0.0, 1.0, 0.0),
		FOV_Y, aspect_ratio);

	let inv_width = 1.0 / (width as f32);
	let inv_height = 1.0 / (height as f32);

	// Test scene (#too: move to Scene)
	let vol = ConstantVolume::new(vec3(0.0, 0.0, 0.0), 2.0, vec3(0.4, 0.1, 0.1), vec3(0.76, 0.35, 0.95));
	let lights: Vec<Box<dyn Light>> = vec![
		Box::new(PointLight { position: vec3(5.0, 0.0, 0.0), intensity: vec3(50.0, 50.0, 100.0) })
	];

	// ----------------------------------------------------------
	// Rendering (#todo: move to renderer)
    for y in 0..height {
        for x in 0..width {
			let u = (x as f32) * inv_width;
			let v = (y as f32) * inv_height;
			let ray = camera.get_ray(u, v);

			let result = integrate_ray(&vol, ray, &lights);

			let mut luminance = result.luminance;
			//let transmittance = result.transmittance;

			// tone mapping
			luminance = vec3(1.0, 1.0, 1.0) - (-luminance * EXPOSURE).exp();
			// gamma correction
			luminance = luminance.pow(1.0 / GAMMA_VALUE);

			rt.set(x as i32, y as i32, luminance);
        }
    }

	print_rendertarget(&rt, FILENAME);

    println!("Output: {}", FILENAME);
}
