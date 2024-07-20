use crate::map_generator::ColorMode;
use three_d::*;

// map
pub const MAP_SIZE: (usize, usize) = (256, 256);
pub const MAPFILE_PATH: &str = "output/mapfile_1";
pub const MAP_COLOR_MODE: ColorMode = ColorMode::HeightMap;
pub const MAP_MAX_HEIGHT: f64 = 100.0;
pub const MAP_MIN_HEIGHT: f64 = 0.0;

// camera initial values
pub const CAM_START_POS: Vec3 = vec3(0.0, 0.0, 10.0);
pub const CAM_START_TARGET: Vec3 = vec3(0.0, 50.0, 0.0);
pub const CAM_START_UP: Vec3 = vec3(0.0, 1.0, 0.0); // maybe always the same up?
pub const CAM_START_FOV: Degrees = degrees(45.0);
pub const CAM_START_Z_NEAR: f32 = 0.1;
pub const CAM_START_Z_FAR: f32 = 1000.0;

// camera movement
pub const CAMERA_MIN_ZOOM: f32 = 10.0;
pub const CAMERA_MAX_ZOOM: f32 = 300.0;
pub const CAMERA_MOVE_SPEED: f32 = 0.5;
pub const CAMERA_SHIFT_FACTOR: f32 = 8.0;
