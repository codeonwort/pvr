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

fn main() {
	// ----------------------------------------------------------
	// Environments (#todo: make configurable)
    let width: usize = 512;
	let height: usize = 512;
	let aspect_ratio = (width as f32) / (height as f32);
	let mut rt: RenderTarget = RenderTarget::new(width, height);

	// ----------------------------------------------------------
	// Modeling (#todo: move to modeler)
	println!("Rasterizing primitives into voxel buffer...");

	let voxel_buffer = VoxelBuffer::new(
		VOXEL_RESOLUTION,
		AABB { min: vec3(-20.0, -20.0, -20.0), max: vec3(20.0, 20.0, 20.0) });
	let mut voxel_volume = VoxelVolume {
		buffer: voxel_buffer,
		emission_value: vec3(0.8, 0.8, 0.8),
		absorption_coeff: vec3(0.20, 0.70, 0.40)
	};
	let point_prim = primitive::point::Point { center: vec3(0.0, 0.0, 0.0), radius: 12.0 };

	point_prim.rasterize(voxel_volume.get_buffer());

	let camera = Camera::new(
		vec3(0.0, 0.0, 30.0), vec3(0.0, 0.0, -1.0), vec3(0.0, 1.0, 0.0),
		FOV_Y, aspect_ratio);

	let inv_width = 1.0 / (width as f32);
	let inv_height = 1.0 / (height as f32);

	// Test scene (#too: move to Scene)
	let constant_volume = ConstantVolume::new(
		vec3(0.0, 0.0, 0.0), 8.0, vec3(0.8, 0.1, 0.2), vec3(0.76, 0.65, 0.95));
	let lights: Vec<Box<dyn Light>> = vec![
		Box::new(PointLight { position: vec3(70.0, 0.0, 20.0), intensity: vec3(1000.0, 1000.0, 1000.0) })
	];

	// ----------------------------------------------------------
	// Rendering (#todo: move to renderer)
	println!("Rendering the voxel buffer...");

	///*
	let mut progress = 0;
	let mut progress_prev = 0;
    for y in 0..height {
        for x in 0..width {
			let u = (x as f32) * inv_width;
			let v = (y as f32) * inv_height;
			let ray = camera.get_ray(u, v);

			let result = integrate_ray(&voxel_volume, ray, &lights);
			let mut luminance = result.luminance;
			//let transmittance = result.transmittance;

			// tone mapping
			luminance = vec3(1.0, 1.0, 1.0) - (-luminance * EXPOSURE).exp();

			// gamma correction
			luminance = luminance.pow(1.0 / GAMMA_VALUE);

			rt.set(x as i32, y as i32, luminance);
		}
		
		progress = (10.0 * (y as f32) / (height as f32)) as i32;
		if progress != progress_prev {
			println!("progress: {} %", progress * 10);
			progress_prev = progress;
		}
	}
	//*/
	
	// noise test
	/*
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
	*/
	
	println!("Printing the image to {}", FILENAME);

	print_rendertarget(&rt, FILENAME);

    println!("Done.");
}
