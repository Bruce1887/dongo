use crate::dongo_entity::dongo_colors::ColorMode;
use three_d::*;

// map
pub const MAP_SEED: u32 = 829394;
pub const MAP_SIZE: (usize, usize) = (512, 512); // num of squares (two triangles) in x and y direction
pub const MAPFILE_PATH: &str = "output/mapfile_1";
pub const MAP_COLOR_MODE: ColorMode = ColorMode::HeightMap;
pub const MAP_MAX_HEIGHT: f64 = 300.0;
pub const MAP_MIN_HEIGHT: f64 = 0.0;
pub const MAP_VERTEX_DISTANCE: f32 = 2.0;
pub const MAP_PERLIN_NOISE_FACTOR: f64 = 1.0; // higher value equals more mountains and valleys

// camera movement
pub const CAMERA_MIN_HEIGHT: f32 = MAP_MIN_HEIGHT as f32 + 100.0;
pub const CAMERA_MAX_HEIGHT: f32 = MAP_MAX_HEIGHT as f32 + 2000.0;
pub const CAMERA_MOVE_SPEED: f32 = 2.0;
pub const CAMERA_SHIFT_FACTOR: f32 = 8.0;
pub const CAMERA_ZOOM_SPEED: f32 = 8.0;
pub const CAMERA_ROTATE_SPEED: f32 = 0.04;

// camera initial values
pub const CAM_START_POS: Vec3 = vec3(0.0, 0.0, (CAMERA_MAX_HEIGHT + CAMERA_MIN_HEIGHT) / 2.0);
pub const CAM_START_TARGET: Vec3 = vec3(0.0, 200.0, 0.0);
pub const CAM_START_UP: Vec3 = vec3(0.0, 1.0, 0.0); // maybe always the same up?
pub const CAM_START_FOV: Degrees = degrees(45.0);
pub const CAM_START_Z_NEAR: f32 = 10.0; // TODO: set this to some better value
pub const CAM_START_Z_FAR: f32 = CAMERA_MAX_HEIGHT * 2.0; // TODO: set this to some better value

// loading indicator stuff
pub fn print_loading_indicator(current: f32, goal: f32) {
    //clear_screen();
    println!("Loading... {:.2}%", current / goal * 100.0);
}

fn _clear_screen() {
    // ANSI escape code to clear the screen
    print!("\x1B[2J\x1B[1;1H");
    // Flush the output to ensure the escape code is executed immediately
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}
