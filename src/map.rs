use macroquad::prelude::{uvec2, vec2, UVec2, Vec2};

use crate::conf::SQUARE_SIZE;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Tile {
    Wall,
    Air,
}

pub struct Map {
    pub map: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}
impl Map {
    pub fn get(&self, loc: &UVec2) -> Tile {
        self.map[loc.y as usize][loc.x as usize]
    }
}

/// Inverse of [world_to_loc]. Converts a location on a [Map] to a world position
pub fn loc_to_world(loc: &UVec2) -> Vec2 {
    vec2(loc.x as f32 * SQUARE_SIZE, loc.y as f32 * SQUARE_SIZE)
}

/// Inverse of [loc_to_world], converts a world position to a location on a [Map]
pub fn world_to_loc(loc: &Vec2) -> UVec2 {
    uvec2((loc.x / SQUARE_SIZE) as u32, (loc.y / SQUARE_SIZE) as u32)
}
