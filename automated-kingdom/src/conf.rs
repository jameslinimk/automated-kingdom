use lazy_static::lazy_static;
use macroquad::text::{load_ttf_font_from_bytes, Font};
use macroquad::window::{screen_height, screen_width};

/// Size of a single tile in pixels
pub const SQUARE_SIZE: f32 = 32.0;

lazy_static! {
    /// Font used for game
    pub static ref SILVER_FONT: Font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/silver.ttf")).unwrap();
}

/// The size of the screen, 1280x720 for small, 1920x1080 for medium, else larger
#[derive(Clone, Copy)]
pub enum ScreenSize {
    Small,
    Medium,
    Large,
}

/// Returns the size of the screen, returns [ScreenSize::Small] if the screen is smaller than 1280x720, [ScreenSize::Medium] if the screen is smaller than 1920x1080, else [ScreenSize::Large]
pub fn screen_size() -> ScreenSize {
    if screen_width() < 1280.0 || screen_height() < 720.0 {
        return ScreenSize::Small;
    }
    if screen_width() < 1920.0 || screen_height() < 1080.0 {
        return ScreenSize::Medium;
    }
    ScreenSize::Large
}

/// Will return `$small` if the screen is smaller than 1280x720, `$medium` if the screen is smaller than 1920x1080, else `$large`
#[macro_export]
macro_rules! screen_size {
    ($small:expr, $medium:expr, $large:expr) => {
        match $crate::conf::screen_size() {
            $crate::conf::ScreenSize::Small => $small,
            $crate::conf::ScreenSize::Medium => $medium,
            $crate::conf::ScreenSize::Large => $large,
        }
    };
}
