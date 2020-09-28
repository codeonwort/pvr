// ----------------------------------------------------------
// standard or 3rd party crates
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

// ----------------------------------------------------------
// module: voxel
mod voxel;
use voxel::VoxelBuffer;

// ----------------------------------------------------------
// module: rendertarget
mod rendertarget;
use rendertarget::Pixel;
use rendertarget::RenderTarget;

// ----------------------------------------------------------
// module: vec3
mod vec3;
use vec3::Vec3;

// ----------------------------------------------------------
// program code
const FILENAME: &str = "test.png";

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

/*fn write_png(filename: &str, buffer: &[u8],
            width: u32, height: u32, color_type: ColorType) {
    let out_file = File::create(filename).unwrap();

    let encoder = PNGEncoder::new(&out_file);
    encoder.encode(&buffer, width, height, color_type).unwrap();

    out_file.sync_all().unwrap();
}*/

fn main() {
    let width: usize = 512;
    let height: usize = 512;
	let mut rt: RenderTarget = RenderTarget::new(width, height); 

	// Test: vec3
	let v1 = Vec3 { x: 5.0, y: 1.0, z: 2.5 };
	let v2 = Vec3 { x: 2.5, y: 3.3, z: 1.0 };
	println!("v1 = {:?}", v1);
	println!("v2 = {:?}", v2);
	println!("-v1 = {:?}", -v1);
	println!("v1 + v2 = {:?}", v1 + v2);
	println!("v1 - v2 = {:?}", v1 - v2);
	println!("v1 * v2 = {:?}", v1 * v2);
	println!("v1 / v2 = {:?}", v1 / v2);
	println!("v1 & v2 = {:?}", v1 & v2);
	println!("v1 ^ v2 = {:?}", v1 ^ v2);
	println!("v1 == v2 = {:?}", v1 == v2);

	// Test: VoxelBuffer
	{
		let mut voxel_buffer = VoxelBuffer::new(10, 10, 10);
		voxel_buffer.write(0, 0, 0, 3.14);
		println!("voxel_buffer[0,0,0] = {}", voxel_buffer.read(0, 0, 0));
	}

    for y in 0..height {
        for x in 0..width {
			let r = (1.0 + (0.1 * x as f32).sin()) * 0.5;
			let g = (1.0 + (0.1 * y as f32).cos()) * 0.5;
			rt.set(x as i32, y as i32, Pixel{ r: r, g: g, b: 0.0} );
        }
    }

	print_rendertarget(&rt, FILENAME);
    //write_png(FILENAME, &buffer, width, height, color_type);

    println!("Output: {}", FILENAME);
}
