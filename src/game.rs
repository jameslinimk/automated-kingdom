use std::sync::{Mutex, MutexGuard};

use derive_new::new;
use lazy_static::lazy_static;

use crate::objects::camera::Camera;
use crate::objects::player::Player;
use crate::objects::worker::update_workers;

lazy_static! {
    pub static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

/// Returns the global [Game] object as a [MutexGuard]
pub fn get_game() -> MutexGuard<'static, Game> {
    GAME.lock().unwrap()
}

#[derive(new)]
pub struct Game {
    #[new(value = "Player::new()")]
    pub player: Player,

    #[new(value = "Camera::new()")]
    pub camera: Camera,
}
impl Game {
    pub fn update(&mut self) {
        self.player.update();
        update_workers();
    }
}
