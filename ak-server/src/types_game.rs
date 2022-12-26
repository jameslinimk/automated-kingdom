use derive_new::new;
use glam::Vec2;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    Blue,
    Red,
    Green,
    Yellow,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Worker {
    pos: Vec2,
}

#[derive(new, Clone, Serialize, Deserialize)]
pub struct Player {
    pub uuid: u64,

    #[new(value = "0")]
    pub ping: u16,

    #[new(value = "vec![]")]
    pub workers: Vec<Worker>,

    pub color: Color,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Tile {
    Wall,
    Air,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Map {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}
impl Map {
    pub fn random() -> Map {
        Map {
            tiles: vec![],
            width: 0,
            height: 0,
        }
    }
}
