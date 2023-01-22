use ak_server::types_game::{Color, Texture};
use derive_new::new;

use crate::conf::SILVER_FONT;
use crate::map::Map;
use crate::objects::camera::Camera;
use crate::objects::player::Player;
use crate::objects::worker::workers_iter_mut;
use crate::texture_map::load_texture;

static mut GAME: Option<Game> = None;
/// Returns the global [Game] object as a mutable reference
pub(crate) fn game() -> &'static mut Game {
    unsafe {
        if GAME.is_none() {
            GAME = Some(Game::new());
        }
        GAME.as_mut().unwrap()
    }
}

#[derive(new)]
pub(crate) struct Game {
    #[new(value = "vec![Player::new()]")]
    pub(crate) players: Vec<Player>,

    #[new(value = "0")]
    pub(crate) main_player: usize,

    #[new(value = "Camera::new()")]
    pub(crate) camera: Camera,

    #[new(value = "Map::new()")]
    pub(crate) map: Map,
}

impl Game {
    pub(crate) fn preload() {
        let _ = *SILVER_FONT;

        /// Loads textures from a list of key-value pairs
        macro_rules! load_textures {
            ($($key:expr => $value:expr,)+) => { load_textures!($($key => $value),+) };
            ($($key:expr => $path:expr),*) => {
                $(
                    load_texture($key, include_bytes!(concat!("../assets/sprites/", $path)));
                )*
            };
        }

        load_textures!(
            Texture::Wall => "wall.png",

            Texture::MiningIcon => "mining_icon.png",
            Texture::GoldPatch => "ores/gold_patch.png",
            Texture::GoldIcon => "ores/gold_icon.png",

            Texture::House => "buildings/house.png",
            Texture::HouseIcon => "buildings/house_icon.png",

            // [code-gen] workers
            Texture::BlueWorkerIcon => "workers/blue/icon.png", Texture::BlueWorkerIdleDown => "workers/blue/idle_down.png", Texture::BlueWorkerIdleUp => "workers/blue/idle_up.png", Texture::BlueWorkerIdleLeft => "workers/blue/idle_left.png", Texture::BlueWorkerIdleRight => "workers/blue/idle_right.png", Texture::BlueWorkerWalkDown => "workers/blue/walk_down.png", Texture::BlueWorkerWalkUp => "workers/blue/walk_up.png", Texture::BlueWorkerWalkLeft => "workers/blue/walk_left.png", Texture::BlueWorkerWalkRight => "workers/blue/walk_right.png",
            Texture::RedWorkerIcon => "workers/red/icon.png", Texture::RedWorkerIdleDown => "workers/red/idle_down.png", Texture::RedWorkerIdleUp => "workers/red/idle_up.png", Texture::RedWorkerIdleLeft => "workers/red/idle_left.png", Texture::RedWorkerIdleRight => "workers/red/idle_right.png", Texture::RedWorkerWalkDown => "workers/red/walk_down.png", Texture::RedWorkerWalkUp => "workers/red/walk_up.png", Texture::RedWorkerWalkLeft => "workers/red/walk_left.png", Texture::RedWorkerWalkRight => "workers/red/walk_right.png",
            Texture::GreenWorkerIcon => "workers/green/icon.png", Texture::GreenWorkerIdleDown => "workers/green/idle_down.png", Texture::GreenWorkerIdleUp => "workers/green/idle_up.png", Texture::GreenWorkerIdleLeft => "workers/green/idle_left.png", Texture::GreenWorkerIdleRight => "workers/green/idle_right.png", Texture::GreenWorkerWalkDown => "workers/green/walk_down.png", Texture::GreenWorkerWalkUp => "workers/green/walk_up.png", Texture::GreenWorkerWalkLeft => "workers/green/walk_left.png", Texture::GreenWorkerWalkRight => "workers/green/walk_right.png",
            Texture::YellowWorkerIcon => "workers/yellow/icon.png", Texture::YellowWorkerIdleDown => "workers/yellow/idle_down.png", Texture::YellowWorkerIdleUp => "workers/yellow/idle_up.png", Texture::YellowWorkerIdleLeft => "workers/yellow/idle_left.png", Texture::YellowWorkerIdleRight => "workers/yellow/idle_right.png", Texture::YellowWorkerWalkDown => "workers/yellow/walk_down.png", Texture::YellowWorkerWalkUp => "workers/yellow/walk_up.png", Texture::YellowWorkerWalkLeft => "workers/yellow/walk_left.png", Texture::YellowWorkerWalkRight => "workers/yellow/walk_right.png",
            // [code-gen] end
        );
    }

    pub(crate) fn init(&mut self) {
        self.map.set_camera_bounds();
    }

    pub(crate) fn update(&mut self) {
        self.map.update();
        self.players[self.main_player].update();
        self.camera.update();
        for worker in workers_iter_mut() {
            worker.update();
        }
    }

    pub(crate) fn draw(&mut self) {
        self.map.draw();
        self.players[self.main_player].draw();
        self.map.draw_minimap();
    }

    pub(crate) fn player(&self, color: Color) -> &Player {
        for player in &self.players {
            if player.color == color {
                return player;
            }
        }
        panic!("Player not found");
    }

    pub(crate) fn player_mut(&mut self, color: Color) -> &mut Player {
        for player in &mut self.players {
            if player.color == color {
                return player;
            }
        }
        panic!("Player not found");
    }

    pub(crate) fn main_player(&self) -> &Player {
        &self.players[self.main_player]
    }

    pub(crate) fn main_player_mut(&mut self) -> &mut Player {
        &mut self.players[self.main_player]
    }
}
