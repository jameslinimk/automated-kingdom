//! Utility functions and macros

use macroquad::prelude::{mouse_position, vec2, Color, Vec2};
use macroquad::text::{draw_text_ex, measure_text, Font, TextParams};
use macroquad::window::{screen_height, screen_width};

use crate::game::get_game;

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
pub fn draw_centered_text(text: &str, x: f32, y: f32, font: Font, font_size: f32, color: Color) {
    let measurements = measure_text(text, Some(font), font_size as u16, 1.0);
    draw_text_ex(
        text,
        x - measurements.width / 2.0,
        y - measurements.height / 2.0,
        TextParams {
            font_size: font_size as u16,
            font_scale: 1.0,
            color,
            font,
            ..Default::default()
        },
    )
}

/// Returns the x position relative to the screen
pub fn rx(x: f32) -> f32 {
    x - (screen_width() / 2.0 - get_game().camera.camera.target.x)
}

/// Returns the y position relative to the screen
pub fn ry(y: f32) -> f32 {
    y - (screen_height() / 2.0 - get_game().camera.camera.target.y)
}

/// Returns the x position relative to the screen (counteracted to adjust for shake)
pub fn rxs(x: f32) -> f32 {
    let shake_offset = if get_game().camera.shake.is_none() {
        0.0
    } else {
        get_game().camera.shake_offset.x
    };
    return x - (screen_width() / 2.0 - get_game().camera.camera.target.x + shake_offset);
}

/// Returns the y position relative to the screen (counteracted to adjust for shake)
pub fn rys(y: f32) -> f32 {
    let shake_offset = if get_game().camera.shake.is_none() {
        0.0
    } else {
        get_game().camera.shake_offset.y
    };
    return y - (screen_height() / 2.0 - get_game().camera.camera.target.y + shake_offset);
}

/// Returns the mouse position relative to the screen
pub fn rel_mouse_pos() -> Vec2 {
    let mouse_pos = mouse_position();
    vec2(rx(mouse_pos.0), ry(mouse_pos.1))
}
