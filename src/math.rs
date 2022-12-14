//! Utility functions for geometry, such as calculating the distance between two points, projecting a location at certain angle, easing functions, etc

use std::f32::consts::PI;

use macroquad::prelude::{vec2, Vec2};

/// Returns the radians between two points
pub fn angle(origin: &Vec2, dest: &Vec2) -> f32 {
    let x_dist = dest.x - origin.x;
    let y_dist = dest.y - origin.y;

    (-y_dist).atan2(x_dist) % (2.0 * PI)
}

/// Gets the opposite radians between two points
pub fn opposite_angle(origin: &Vec2, dest: &Vec2) -> f32 {
    let x_dist = origin.x - dest.x;
    let y_dist = origin.y - dest.y;

    (-y_dist).atan2(x_dist) % (2.0 * PI)
}

/// Projects point at certain radians and distance
pub fn project(origin: &Vec2, radians: f32, distance: f32) -> Vec2 {
    vec2(
        origin.x + (radians.cos() * distance),
        origin.y - (radians.sin() * distance),
    )
}

/// Finds distance between 2 points
pub fn distance(p1: &Vec2, p2: &Vec2) -> f32 {
    ((p1.x - p2.x).powf(2.0) + (p1.y - p2.y).powf(2.0)).sqrt()
}

/// Converts a value from `0.0` - `1.0` to an ease-in-out curve (sign wave)
pub fn ease_in_out(x: f32) -> f32 {
    (-((PI * x).cos() - 1.0) / 2.0).clamp(0.0, 1.0)
}
