use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    Blue,
    Red,
    Green,
    Yellow,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerWorker {
    pub pos: (f32, f32),
    pub sprite: String,
}

#[derive(new, Clone, Serialize, Deserialize)]
pub struct ServerPlayer {
    pub uuid: u64,

    #[new(value = "0")]
    pub ping: u16,

    #[new(value = "vec![]")]
    pub workers: Vec<ServerWorker>,

    pub color: Color,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Tile {
    Wall,
    Air,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerMap {
    pub tiles: Vec<Vec<Tile>>,
    pub width: usize,
    pub height: usize,
}
impl ServerMap {
    pub fn random() -> ServerMap {
        ServerMap {
            tiles: vec![],
            width: 0,
            height: 0,
        }
    }
}
