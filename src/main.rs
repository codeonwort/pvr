// #todo: Separate image manager
use image::png::PNGEncoder;
use image::ColorType;
use std::fs::File;

const FILENAME: &str = "test.png";

fn main() {
    println!("Hello, world!");

    let mut buffer: Vec<u8> = Vec::new();
    let width: u32 = 512;
    let height: u32 = 512;
    let color_type: ColorType = ColorType::Rgb8;
    let channels: u32 = 3;

    let buffer_size = (width * height * channels) as usize;
    buffer.resize(buffer_size, 0);

    let mut ptr: usize = 0;
    for _y in 0..height-1 {
        for _x in 0..width-1 {
            buffer[ptr] = 0xff;
            buffer[ptr+1] = 0;
            buffer[ptr+2] = 0;
            ptr += 3;
        }
    }

    let out_file = File::create(FILENAME).unwrap();

    let encoder = PNGEncoder::new(&out_file);
    encoder.encode(&buffer, width, height, color_type).unwrap();

    out_file.sync_all().unwrap();
}
