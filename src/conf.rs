use macroquad::text::{load_ttf_font_from_bytes, Font};

/// Size of a single tile in pixels
pub const SQUARE_SIZE: f32 = 32.0;

macro_rules! font {
    ($raw_name: ident, $getter_name: ident, $font_path: expr) => {
        static mut $raw_name: Option<Font> = None;
        pub fn $getter_name() -> Font {
            unsafe {
                if $raw_name.is_none() {
                    $raw_name = Some(load_ttf_font_from_bytes(include_bytes!($font_path)).unwrap());
                }
                $raw_name.unwrap()
            }
        }
    };
}

font!(R1, get_font, "../assets/fonts/8bit.ttf");
font!(R2, get_font_small, "../assets/fonts/8bit_small.ttf");
