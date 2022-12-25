use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Color {
    Blue,
    Red,
    Green,
    Yellow,
}

#[derive(Serialize, Deserialize)]
pub struct Worker {
    pos: Vec2,
}

#[derive(Serialize, Deserialize)]
pub struct Player {
    pub uuid: u64,
    pub ping: u16,
    pub workers: Vec<Worker>,
    pub color: Color,
}

#[derive(Serialize, Deserialize)]
pub enum Tile {
    Wall,
    Air,
}

#[derive(Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}
