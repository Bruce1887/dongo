use crate::map_generator::ColorMode;
use three_d::*;

// map
pub const MAP_SIZE: (usize, usize) = (256, 256);
pub const MAPFILE_PATH: &str = "output/mapfile_1";

pub const MAP_COLOR_MODE: ColorMode = ColorMode::HeightMap;
pub const MAP_MAX_HEIGHT: f64 = 100.0;
pub const MAP_MIN_HEIGHT: f64 = 0.0;

// camera initial values
pub const CAM_START_POS: Vec3 = vec3(0.0, 0.0, (CAMERA_MAX_HEIGHT + CAMERA_MIN_HEIGHT) / 2.0);
pub const CAM_START_TARGET: Vec3 = vec3(0.0, 50.0, 0.0);
pub const CAM_START_UP: Vec3 = vec3(0.0, 1.0, 0.0); // maybe always the same up?
pub const CAM_START_FOV: Degrees = degrees(45.0);
pub const CAM_START_Z_NEAR: f32 = 0.1; // TODO: set this to some better value
pub const CAM_START_Z_FAR: f32 = 1000.0; // TODO: set this to some better value

// camera movement
pub const CAMERA_MIN_HEIGHT: f32 = MAP_MIN_HEIGHT as f32 + 10.0;
pub const CAMERA_MAX_HEIGHT: f32 = MAP_MAX_HEIGHT as f32 + 500.0;
pub const CAMERA_MOVE_SPEED: f32 = 0.5;
pub const CAMERA_SHIFT_FACTOR: f32 = 8.0;




// loading indicator stuff
pub fn print_loading_indicator(current: f32, goal: f32){
    //clear_screen();
    println!("Loading... {:.2}%", current / goal * 100.0);
}   

fn _clear_screen() {
    // ANSI escape code to clear the screen
    print!("\x1B[2J\x1B[1;1H");
    // Flush the output to ensure the escape code is executed immediately
    std::io::Write::flush(&mut std::io::stdout()).unwrap();
}