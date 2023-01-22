//! Utility functions and macros

use macroquad::prelude::{mouse_position, vec2, Color, Vec2, WHITE};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};
use macroquad::text::{draw_text_ex, measure_text, TextParams};
use macroquad::texture::{draw_texture, draw_texture_ex, DrawTextureParams, Texture2D};

use crate::conf::SILVER_FONT;
use crate::game::game;
use crate::geometry::CollisionRect;

/// Create [rustc_hash::FxHashMap]'s using a readable syntax, similar to dicts in python or objects in js. Adapted from maplit to support `FxHashMap`
///
/// ## Example
///
/// ```
/// let map = hashmap!{
///     "a" => 1,
///     "b" => 2,
/// };
/// assert_eq!(map["a"], 1);
/// assert_eq!(map["b"], 2);
/// assert_eq!(map.get("c"), None);
/// ```
#[macro_export]
macro_rules! hashmap {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashmap!(@single $rest)),*]));

    ($($key:expr => $value:expr,)+) => { $crate::hashmap!($($key => $value),+) };
    ($($key:expr => $value:expr),*) => {
        {
            let _cap = $crate::hashmap!(@count $($key),*);
            let mut _map = rustc_hash::FxHashMap::with_capacity_and_hasher(_cap, Default::default());
            $(
                let _ = _map.insert($key, $value);
            )*
            _map
        }
    };
}

/// Create [rustc_hash::FxHashSet]'s using a readable syntax. Adapted from maplit to support `FxHashSet`
///
/// ## Example
///
/// ```
/// let set = hashset!{"a", "b"};
/// assert!(set.contains("a"));
/// assert!(set.contains("b"));
/// assert!(!set.contains("c"));
/// ```
#[macro_export]
macro_rules! hashset {
    (@single $($x:tt)*) => (());
    (@count $($rest:expr),*) => (<[()]>::len(&[$(hashset!(@single $rest)),*]));

    ($($key:expr,)+) => { $crate::hashset!($($key),+) };
    ($($key:expr),*) => {
        {
            let _cap = $crate::hashset!(@count $($key),*);
            let mut _set = rustc_hash::FxHashSet::with_capacity_and_hasher(_cap, Default::default());
            $(
                let _ = _set.insert($key);
            )*
            _set
        }
    };
}

/// Trait to fix `f32.signum` not doing `0.0`
pub(crate) trait FloatSignum {
    fn sign(&self) -> f32;
    fn sign_i8(&self) -> i8;
}
impl FloatSignum for f32 {
    /// Returns `0.0` if `self` is `0.0`, `1.0` if `self` is positive, and `-1.0` if `self` is negative
    fn sign(&self) -> f32 {
        if *self > 0.0 {
            1.0
        } else if *self < 0.0 {
            -1.0
        } else {
            0.0
        }
    }

    /// Returns `0.0` if `self` is `0.0`, `1.0` if `self` is positive, and `-1.0` if `self` is negative but as an [i8]
    fn sign_i8(&self) -> i8 {
        if *self > 0.0 {
            1
        } else if *self < 0.0 {
            -1
        } else {
            0
        }
    }
}

/// Derives [PartialEq] for a struct, comparing only the `id` field
#[macro_export]
macro_rules! derive_id_eq {
    ($struct:ident) => {
        impl PartialEq for $struct {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }
    };
}

/// Draw text centered at a given position
pub(crate) fn draw_text_center(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let measurements = measure_text(text, Some(*SILVER_FONT), font_size as u16, 1.0);
    draw_text_ex(
        text,
        x - measurements.width / 2.0,
        y + font_size / 4.0,
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.0,
            color,
            font: *SILVER_FONT,
            ..Default::default()
        },
    )
}

/// Draw text at given top left position
pub(crate) fn draw_text_top_left(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    draw_text_ex(
        text,
        x,
        y + font_size / 2.0,
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.0,
            color,
            font: *SILVER_FONT,
            ..Default::default()
        },
    )
}

/// Draw text at given top left position
pub(crate) fn draw_rel_text_top_left(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    draw_text_ex(
        text,
        relative_x(x),
        relative_y(y + font_size / 2.0),
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.0 / game().camera.zoom,
            color,
            font: *SILVER_FONT,
            ..Default::default()
        },
    )
}

/// Draws text within a given rectangle, wrapping the text to fit the rectangle
pub(crate) fn draw_text_within_rect(
    text: &str,
    rect: &CollisionRect,
    font_size: f32,
    color: Color,
) {
    let mut x = rect.left();
    let mut y = rect.top();

    for word in text.split_whitespace() {
        let word = word.to_string() + " ";
        let width = measure_text(&word, Some(*SILVER_FONT), font_size as u16, 1.0).width;

        if x + width > rect.left() + rect.width {
            // Move to the next line
            x = rect.left();
            y += font_size;
        }

        draw_text_top_left(&word, x, y, font_size, color);

        x += width;
    }
}

/// Returns a position relative to the screen
pub(crate) fn relative_pos(pos: Vec2) -> Vec2 {
    game().camera.camera.screen_to_world(pos)
}

/// Returns the x position relative to the screen
pub(crate) fn relative_x(x: f32) -> f32 {
    game().camera.camera.screen_to_world(vec2(x, 0.0)).x
}

/// Returns the y position relative to the screen
pub(crate) fn relative_y(y: f32) -> f32 {
    game().camera.camera.screen_to_world(vec2(0.0, y)).y
}

/// Returns the given value without zoom
pub(crate) fn relative_zoom(v: f32) -> f32 {
    v / game().camera.zoom
}

/// Returns the given [Vec2] without zoom
pub(crate) fn relative_zoom_vec2(v: Vec2) -> Vec2 {
    v / game().camera.zoom
}

/// Draw a rectangle relative to the screen
pub(crate) fn draw_rel_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color) {
    draw_rectangle(
        relative_x(x),
        relative_y(y),
        relative_zoom(w),
        relative_zoom(h),
        color,
    );
}

/// Draw rectangle lines relative to the screen
pub(crate) fn draw_rel_rectangle_lines(
    x: f32,
    y: f32,
    w: f32,
    h: f32,
    thickness: f32,
    color: Color,
) {
    draw_rectangle_lines(
        relative_x(x),
        relative_y(y),
        relative_zoom(w),
        relative_zoom(h),
        thickness,
        color,
    );
}

/// Draw a texture relative to the screen with extra params
pub(crate) fn draw_rel_texture_ex(texture: Texture2D, x: f32, y: f32, params: DrawTextureParams) {
    draw_texture_ex(
        texture,
        relative_x(x),
        relative_y(y),
        WHITE,
        DrawTextureParams {
            dest_size: params
                .dest_size
                .map(relative_zoom_vec2)
                .or_else(|| Some(relative_zoom_vec2(vec2(texture.width(), texture.height())))),
            ..params
        },
    );
}

/// Draw a texture relative to the screen
pub(crate) fn draw_texture_center(texture: Texture2D, x: f32, y: f32) {
    draw_texture(
        texture,
        x - texture.width() / 2.0,
        y - texture.height() / 2.0,
        WHITE,
    );
}

/// Draw a texture relative to the screen with extra params
pub(crate) fn draw_texture_center_ex(
    texture: Texture2D,
    x: f32,
    y: f32,
    params: DrawTextureParams,
) {
    let width;
    let height;

    if let Some(size) = params.dest_size {
        width = size.x;
        height = size.y;
    } else {
        width = texture.width();
        height = texture.height();
    }

    draw_texture_ex(texture, x - width / 2.0, y - height / 2.0, WHITE, params);
}

/// Returns the mouse position relative to the screen
pub(crate) fn screen_mouse_pos() -> Vec2 {
    relative_pos(mouse_position().into())
}

/// Returns the mouse pos as a [Vec2]
pub(crate) fn mouse_pos() -> Vec2 {
    mouse_position().into()
}

/// Macro for creating a [Color] from a hex code
#[macro_export]
macro_rules! hex {
    ($hex:expr) => {{
        let r = u8::from_str_radix(&$hex[1..3], 16).unwrap();
        let g = u8::from_str_radix(&$hex[3..5], 16).unwrap();
        let b = u8::from_str_radix(&$hex[5..7], 16).unwrap();
        macroquad::prelude::Color::from_rgba(r, g, b, 255)
    }};
}

/// Ternary macro
#[macro_export]
macro_rules! ternary {
    ($cond:expr, $if:expr, $else:expr) => {
        if $cond {
            $if
        } else {
            $else
        }
    };
}

/// Abbreviates a number to a more readable format
///
/// # Examples
///
/// ```rs
/// 1             => "1"
/// 123           => "123"
/// 1_230         => "1.23k"
/// 1_000_000     => "1.00m"
/// 1_000_000_000 => "1.00b"
/// ```
pub(crate) fn abbreviate_number(num: u32) -> String {
    let num_string = num.to_string();
    let mut output = String::new();

    let (suffix, divider) = match num_string.len() {
        n if n > 9 => ('b', 1000000000.0),
        n if n > 6 => ('m', 1000000.0),
        n if n > 3 => ('k', 1000.0),
        _ => (' ', 1.0),
    };

    let decimal = (num as f64) / divider;
    if decimal.fract() > 0.0 {
        output.push_str(&format!("{:.2}", decimal));
    } else {
        output.push_str(&format!("{}", decimal.trunc()));
    }

    if suffix != ' ' {
        output.push(suffix);
    }

    output
}
