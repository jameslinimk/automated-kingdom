//! Utility functions for geometry, such as calculating the distance between two points, projecting a location at certain angle, easing functions, etc

use std::f32::consts::PI;

use macroquad::prelude::{vec2, UVec2, Vec2};

/// Returns the radians between two [Vec2]'s
pub fn angle(origin: &Vec2, dest: &Vec2) -> f32 {
    let x_dist = dest.x - origin.x;
    let y_dist = dest.y - origin.y;

    (-y_dist).atan2(x_dist) % (2.0 * PI)
}

/// Gets the opposite radians between two [Vec2]'s
pub fn opposite_angle(origin: &Vec2, dest: &Vec2) -> f32 {
    let x_dist = origin.x - dest.x;
    let y_dist = origin.y - dest.y;

    (-y_dist).atan2(x_dist) % (2.0 * PI)
}

/// Finds distance between 2 [Vec2]'s
pub fn distance(p1: &Vec2, p2: &Vec2) -> f32 {
    let x_dist = p2.x - p1.x;
    let y_dist = p2.y - p1.y;

    (x_dist * x_dist + y_dist * y_dist).sqrt()
}

/// Finds distance between 2 [UVec2]'s
pub fn u_distance(p1: &UVec2, p2: &UVec2) -> f32 {
    let x_dist = p2.x as f32 - p1.x as f32;
    let y_dist = p2.y as f32 - p1.y as f32;

    (x_dist * x_dist + y_dist * y_dist).sqrt()
}

/// Projects [Vec2] at certain radians and distance
pub fn project(origin: &Vec2, radians: f32, distance: f32) -> Vec2 {
    vec2(
        origin.x + (radians.cos() * distance),
        origin.y - (radians.sin() * distance),
    )
}

/// Converts a value from `0.0` - `1.0` to an ease-in-out curve (sign wave)
pub fn ease_in_out(x: f32) -> f32 {
    (-((PI * x).cos() - 1.0) / 2.0).clamp(0.0, 1.0)
}
