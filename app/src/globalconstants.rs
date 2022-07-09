// ----------------------------------------------------------
// Global constants accessed from main and app
// ----------------------------------------------------------

pub const WINDOW_TITLE: &str = "PVR GUI";
pub const WINDOW_WIDTH: f64 = 1280.0;
pub const WINDOW_HEIGHT: f64 = 768.0;

pub const IMAGE_WIDTH: usize = 512;
pub const IMAGE_HEIGHT: usize = 512;
pub const FILENAME: &str = "output.png";

// Default renderer settings
pub const EXPOSURE: f32 = 1.2;
pub const GAMMA_VALUE: f32 = 2.2;
// #todo: Step sizes are too coarse. 0.25 would be nice but it's 16x times slower.
pub const STEP_SIZE_1ST: f32 = 1.0;
pub const STEP_SIZE_2ND: f32 = 1.0;
pub const FOV_Y: f32 = 45.0;
pub const VOXEL_RESOLUTION: (i32, i32, i32) = (512, 512, 256);
