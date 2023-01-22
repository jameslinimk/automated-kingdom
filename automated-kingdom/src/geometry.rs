//! Contains helper functions and structs for working with rectangles, points, and polygons

use derive_new::new;
use macroquad::prelude::{vec2, Color, Vec2};
use macroquad::shapes::{draw_rectangle, draw_rectangle_lines};

use crate::util::{relative_pos, relative_zoom};

/// Wrapper for a rectangle, used for collision detection and other things
#[derive(Debug, Clone, Copy, PartialEq, new)]
pub(crate) struct CollisionRect {
    /// The x coordinate of the top left corner of the rectangle
    x: f32,
    /// The y coordinate of the top left corner of the rectangle
    y: f32,
    /// The width of the rectangle
    pub(crate) width: f32,
    /// The height of the rectangle
    pub(crate) height: f32,
}
impl CollisionRect {
    /// Creates a new [CollisionRect] with the given center and size
    pub(crate) fn new_center(center: Vec2, width: f32, height: f32) -> CollisionRect {
        CollisionRect::new(
            center.x - width / 2.0,
            center.y - height / 2.0,
            width,
            height,
        )
    }

    /// Creates a new [CollisionRect] with the given center and size, relative to the screen
    pub(crate) fn new_rel_center(center: Vec2, width: f32, height: f32) -> CollisionRect {
        CollisionRect::new_rel(
            center.x - width / 2.0,
            center.y - height / 2.0,
            width,
            height,
        )
    }

    /// Creates a new [CollisionRect] with the given top left corner and size, relative to the screen
    pub(crate) fn new_rel(x: f32, y: f32, width: f32, height: f32) -> CollisionRect {
        CollisionRect::new_vec2(
            relative_pos(vec2(x, y)),
            relative_zoom(width),
            relative_zoom(height),
        )
    }

    /// Creates a new [CollisionRect] with the given top left corner and size
    pub(crate) fn new_vec2(top_left: Vec2, width: f32, height: f32) -> CollisionRect {
        CollisionRect::new(top_left.x, top_left.y, width, height)
    }

    /// Draws the rectangle to the screen
    pub(crate) fn draw(&self, color: Color) {
        draw_rectangle(self.x, self.y, self.width, self.height, color);
    }

    /// Draws the rectangle to the screen with a border
    pub(crate) fn draw_lines(&self, thickness: f32, color: Color) {
        draw_rectangle_lines(self.x, self.y, self.width, self.height, thickness, color);
    }

    /// Sees if `self` touches `other`
    pub(crate) fn touches_rect(&self, other: &CollisionRect) -> bool {
        self.x < other.x + other.width
            && self.x + self.width > other.x
            && self.y < other.y + other.height
            && self.y + self.height > other.y
    }

    /// Sees if `self` touches `point`
    pub(crate) fn touches_point(&self, point: &Vec2) -> bool {
        self.x < point.x
            && self.x + self.width > point.x
            && self.y < point.y
            && self.y + self.height > point.y
    }

    /// Returns the left side of the rectangle
    pub(crate) fn left(&self) -> f32 {
        self.x
    }

    /// Returns the right side of the rectangle
    pub(crate) fn right(&self) -> f32 {
        self.x + self.width
    }

    /// Returns the top side of the rectangle
    pub(crate) fn top(&self) -> f32 {
        self.y
    }

    /// Returns the bottom side of the rectangle
    pub(crate) fn bottom(&self) -> f32 {
        self.y + self.height
    }

    /// Returns the center of the rectangle
    pub(crate) fn center(&self) -> Vec2 {
        vec2(self.x + self.width / 2.0, self.y + self.height / 2.0)
    }

    /// Returns the top left corner of the rectangle
    pub(crate) fn top_left(&self) -> Vec2 {
        vec2(self.x, self.y)
    }

    /// Returns the top right corner of the rectangle
    pub(crate) fn top_right(&self) -> Vec2 {
        vec2(self.x + self.width, self.y)
    }

    /// Returns the bottom left corner of the rectangle
    pub(crate) fn bottom_left(&self) -> Vec2 {
        vec2(self.x, self.y + self.height)
    }

    /// Returns the bottom right corner of the rectangle
    pub(crate) fn bottom_right(&self) -> Vec2 {
        vec2(self.x + self.width, self.y + self.height)
    }

    /// Sets the center of the rectangle
    pub(crate) fn set_center(&mut self, center: Vec2) {
        self.x = center.x - self.width / 2.0;
        self.y = center.y - self.height / 2.0;
    }

    /// Sets the top left corner of the rectangle
    pub(crate) fn set_top_left(&mut self, top_left: Vec2) {
        self.x = top_left.x;
        self.y = top_left.y;
    }

    /// Sets the top right corner of the rectangle
    pub(crate) fn set_top_right(&mut self, top_right: Vec2) {
        self.x = top_right.x - self.width;
        self.y = top_right.y;
    }

    /// Sets the bottom left corner of the rectangle
    pub(crate) fn set_bottom_left(&mut self, bottom_left: Vec2) {
        self.x = bottom_left.x;
        self.y = bottom_left.y - self.height;
    }

    /// Sets the bottom right corner of the rectangle
    pub(crate) fn set_bottom_right(&mut self, bottom_right: Vec2) {
        self.x = bottom_right.x - self.width;
        self.y = bottom_right.y - self.height;
    }

    /// Sets the left side of the rectangle
    pub(crate) fn set_left(&mut self, left: f32) {
        self.x = left;
    }

    /// Sets the right side of the rectangle
    pub(crate) fn set_right(&mut self, right: f32) {
        self.x = right - self.width;
    }

    /// Sets the top side of the rectangle
    pub(crate) fn set_top(&mut self, top: f32) {
        self.y = top;
    }

    /// Sets the bottom side of the rectangle
    pub(crate) fn set_bottom(&mut self, bottom: f32) {
        self.y = bottom - self.height;
    }

    /// Will expand the rectangle by the given amount in each direction, keeping the center the same
    pub(crate) fn expand_center(&mut self, width_diff: f32, height_diff: f32) {
        let center = self.center();
        self.width += width_diff;
        self.height += height_diff;
        self.set_center(center);
    }
}
