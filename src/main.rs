#![feature(option_result_contains)]

use macroquad::window::{next_frame, Conf};

use crate::game::get_game;

pub mod astar;
pub mod conf;
pub mod game;
pub mod geometry;
pub mod map;
pub mod math;
pub mod objects;
pub mod util;

/// Config for the game
fn config() -> Conf {
    Conf {
        window_title: "Futuretes".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    loop {
        get_game().update();
        get_game().draw();
        next_frame().await;
    }
}
