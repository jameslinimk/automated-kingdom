use lazy_static::lazy_static;
use macroquad::text::{load_ttf_font_from_bytes, Font};

/// Size of a single tile in pixels
pub const SQUARE_SIZE: f32 = 32.0;

lazy_static! {
    /// Font used for game
    pub static ref SILVER_FONT: Font =
        load_ttf_font_from_bytes(include_bytes!("../assets/fonts/silver.ttf")).unwrap();
}
