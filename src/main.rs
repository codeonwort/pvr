// ----------------------------------------------------------
// standard or 3rd party crates
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

// ----------------------------------------------------------
// module: voxel and volume
mod voxel;
mod volume;
use voxel::VoxelBuffer;
use volume::Volume;
use volume::ConstantVolume;

// ----------------------------------------------------------
// module: rendertarget
mod rendertarget;
use rendertarget::Pixel;
use rendertarget::RenderTarget;

// ----------------------------------------------------------
// module: vec3, ray, and camera
// #todo-module: This is definitely going weird. Cleanup mod imports.
mod vec3;
mod ray;
mod camera;
use vec3::Vec3;
use ray::Ray;
use camera::Camera;

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

fn integrate_emission(vol: &Volume, ray: Ray) -> Vec3 {
	// #todo: proper step size
	let step_size: f32 = 0.25;

	// Integration bounds
	let interval = vol.get_intersection(ray);
	match interval {
		None => Vec3::new(0.0, 0.0, 0.0),
		Some((t_start, t_end)) => {
			//let step_size: f32 = (t_end - t_start) / 20.0;
			let mut t_current = t_start;
			let mut T: f32 = 1.0;
			let mut L: Vec3 = Vec3::new(0.0, 0.0, 0.0);
		
			while t_current < t_end {
				let p_i: Vec3 = ray.at(t_current);
				let Le: Vec3 = vol.emission(p_i);
				let sigma_a: f32 = vol.absorption(p_i);
				let T_i: f32 = (-sigma_a * step_size).exp();
				T *= T_i;
				L = L + T * Le;
				t_current += step_size;
			}
		
			L
		}
	}
}

fn main() {
    let width: usize = 512;
	let height: usize = 512;
	let aspect_ratio = (width as f32) / (height as f32);
	let mut rt: RenderTarget = RenderTarget::new(width, height);

	// Test: VoxelBuffer
	{
		let mut voxel_buffer = VoxelBuffer::new(10, 10, 10);
		voxel_buffer.write(0, 0, 0, 3.14);
		println!("voxel_buffer[0,0,0] = {}", voxel_buffer.read(0, 0, 0));
	}

	let camera = Camera::new(
		Vec3::new(0.0, 0.0, -10.0), Vec3::new(0.0, 0.0, 10.0), Vec3::new(0.0, 1.0, 0.0),
		FOV_Y, aspect_ratio);

	let inv_width = 1.0 / (width as f32);
	let inv_height = 1.0 / (height as f32);

	let vol = ConstantVolume::new(Vec3::new(0.0, 0.0, 0.0), 2.0, Vec3::new(0.7, 0.15, 0.05), 0.7);

    for y in 0..height {
        for x in 0..width {
			let u = (x as f32) * inv_width;
			let v = (y as f32) * inv_height;

			let ray = camera.get_ray(u, v);
			let mut final_color = integrate_emission(&vol, ray);

			// tone mapping
			final_color = Vec3::new(1.0, 1.0, 1.0) - (-final_color * EXPOSURE).exp();

			// gamma correction
			final_color = final_color.pow(1.0 / GAMMA_VALUE);

			rt.set(x as i32, y as i32, Pixel { r: final_color.x, g: final_color.y, b: final_color.z });
        }
    }

	print_rendertarget(&rt, FILENAME);

    println!("Output: {}", FILENAME);
}
