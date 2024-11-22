use three_d::Srgba;

#[allow(dead_code)]
pub enum ColorMode {
    HeightMap,
    Checkerboard,
}

// Colors
pub const DONGOCOLOR_RED: Srgba = Srgba::new(150, 0, 0, 255);
pub const DONGOCOLOR_GREEN: Srgba = Srgba::new(0, 155, 0, 255);
pub const DONGOCOLOR_DARKGREEN: Srgba = Srgba::new(0, 100, 0, 255);
pub const DONGOCOLOR_BROWN: Srgba = Srgba::new(70, 70, 0, 255);
pub const DONGOCOLOR_BLUE: Srgba = Srgba::new(0, 0, 255, 255);
pub const DONGOCOLOR_YELLOW: Srgba = Srgba::new(255, 255, 0, 255);
pub const DONGOCOLOR_CYAN: Srgba = Srgba::new(0, 255, 255, 255);
pub const DONGOCOLOR_MAGENTA: Srgba = Srgba::new(255, 0, 255, 255);
pub const DONGOCOLOR_BLACK: Srgba = Srgba::new(0, 0, 0, 255);
pub const DONGOCOLOR_WHITE: Srgba = Srgba::new(255, 255, 255, 255);
pub const DONGOCOLOR_DARKWHITE: Srgba = Srgba::new(200, 200, 200, 255);
pub const DONGOCOLOR_GRAY: Srgba = Srgba::new(100, 100, 100, 255);