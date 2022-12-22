//! Utility functions and macros

use macroquad::prelude::{mouse_position, vec2, Color, Vec2};
use macroquad::shapes::draw_rectangle;
use macroquad::text::{draw_text_ex, measure_text, TextParams};

use crate::conf::SILVER_FONT;
use crate::game::game;

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

/// Draw text centered at a given position
pub fn draw_centered_text(text: &str, x: f32, y: f32, font_size: f32, color: Color) {
    let measurements = measure_text(text, Some(*SILVER_FONT), font_size as u16, 1.0);
    draw_text_ex(
        text,
        x - measurements.width / 2.0,
        y - measurements.height / 2.0,
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.0,
            color,
            font: *SILVER_FONT,
            ..Default::default()
        },
    );
}

// Returns a position relative to the screen
pub fn relative(pos: Vec2) -> Vec2 {
    game().camera.camera.screen_to_world(pos)
}

/// Returns the x position relative to the screen
pub fn relative_x(x: f32) -> f32 {
    game().camera.camera.screen_to_world(vec2(x, 0.0)).x
}

/// Returns the y position relative to the screen
pub fn relative_y(y: f32) -> f32 {
    game().camera.camera.screen_to_world(vec2(0.0, y)).y
}

/// Returns the given value without zoom
pub fn relative_zoom(v: f32) -> f32 {
    v / game().camera.zoom
}

/// Draw a rectangle relative to the screen
pub fn draw_rel_rectangle(x: f32, y: f32, w: f32, h: f32, color: Color) {
    draw_rectangle(
        relative_x(x),
        relative_y(y),
        relative_zoom(w),
        relative_zoom(h),
        color,
    );
}

/// Returns the mouse position relative to the screen
pub fn rel_mouse_pos() -> Vec2 {
    relative(mouse_position().into())
}
