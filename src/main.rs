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

	// Test: VoxelBuffer
	{
		let mut voxel_buffer = VoxelBuffer::new(10, 10, 10);
		voxel_buffer.write(0, 0, 0, 3.14);
		println!("voxel_buffer[0,0,0] = {}", voxel_buffer.read(0, 0, 0));
	}

    for y in 0..height {
        for x in 0..width {
			rt.set(x as i32, y as i32, Pixel{ r: 1.0, g: 1.0, b: 1.0} );
        }
    }

	print_rendertarget(&rt, FILENAME);
    //write_png(FILENAME, &buffer, width, height, color_type);

    println!("Output: {}", FILENAME);
}

