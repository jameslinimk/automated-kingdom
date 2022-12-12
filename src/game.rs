use std::sync::{Mutex, MutexGuard};

use derive_new::new;
use lazy_static::lazy_static;
use macroquad::prelude::RED;
use macroquad::shapes::draw_rectangle;

lazy_static! {
    pub static ref GAME: Mutex<Game> = Mutex::new(Game::new());
}

pub fn get_game() -> MutexGuard<'static, Game> {
    GAME.lock().unwrap()
}

#[derive(new)]
pub struct Game {}
impl Game {
    pub fn update(&self) {
        draw_rectangle(0.0, 0.0, 100.0, 100.0, RED);
    }
}
