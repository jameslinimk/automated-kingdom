use derive_new::new;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum Color {
    Blue,
    Red,
    Green,
    Yellow,
}

#[rustfmt::skip]
#[derive(Deserialize, Serialize, Clone, Copy, Debug, Hash, PartialEq, Eq)]
pub enum Texture {
    Wall,

    // [code-gen] workers
    BlueWorkerIcon, BlueWorkerIdleDown, BlueWorkerIdleUp, BlueWorkerIdleLeft, BlueWorkerIdleRight, BlueWorkerWalkDown, BlueWorkerWalkUp, BlueWorkerWalkLeft, BlueWorkerWalkRight,
    RedWorkerIcon, RedWorkerIdleDown, RedWorkerIdleUp, RedWorkerIdleLeft, RedWorkerIdleRight, RedWorkerWalkDown, RedWorkerWalkUp, RedWorkerWalkLeft, RedWorkerWalkRight,
    GreenWorkerIcon, GreenWorkerIdleDown, GreenWorkerIdleUp, GreenWorkerIdleLeft, GreenWorkerIdleRight, GreenWorkerWalkDown, GreenWorkerWalkUp, GreenWorkerWalkLeft, GreenWorkerWalkRight,
    YellowWorkerIcon, YellowWorkerIdleDown, YellowWorkerIdleUp, YellowWorkerIdleLeft, YellowWorkerIdleRight, YellowWorkerWalkDown, YellowWorkerWalkUp, YellowWorkerWalkLeft, YellowWorkerWalkRight,
    // [code-gen] end
}
impl Texture {
    /// Returns the texture as a [Sprite]
    pub fn as_server(&self) -> Sprite {
        Sprite::Sprite(*self)
    }
}

/// A texture or a spritesheet, used to transmit textures to the client and server
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Sprite {
    Sprite(Texture),
    SpriteSheet {
        /// Base texture
        texture: Texture,
        /// Amount of frames in the spritesheet
        frames: u16,
        /// Current frame of the spritesheet
        current_frame: u16,
    },
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ServerWorker {
    pub pos: (f32, f32),
    pub sprite: Sprite,
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