// ----------------------------------------------------------
// Global constants accessed from main and app
// ----------------------------------------------------------

use pvrlib::math::vec3::*;

pub const WINDOW_TITLE: &str = "PVR GUI";
pub const WINDOW_WIDTH: f64 = 1600.0;
pub const WINDOW_HEIGHT: f64 = 900.0;

pub const IMAGE_WIDTH: usize = 512;
pub const IMAGE_HEIGHT: usize = 512;
pub const FILENAME_PNG: &str = "output.png";
pub const FILENAME_JPG: &str = "output.jpg";

// Default renderer settings
pub const WORK_GROUP_SIZE: (usize, usize) = (16, 16);
pub const EXPOSURE: f32 = 1.2;
pub const GAMMA_VALUE: f32 = 2.2;
// #todo: Step sizes are too coarse. 0.25 would be nice but it's 16x times slower.
pub const STEP_SIZE_1ST: f32 = 1.0;
pub const STEP_SIZE_2ND: f32 = 1.0;
pub const CAMERA_ORIGIN: vec3f = vec3f { x:0.0, y:0.0, z:50.0 };
pub const CAMERA_LOOKAT: vec3f = vec3f { x:-15.0, y:10.0, z:0.0 };
pub const FOV_Y: f32 = 45.0;
pub const VOXEL_RESOLUTION: (i32, i32, i32) = (512, 512, 256);
