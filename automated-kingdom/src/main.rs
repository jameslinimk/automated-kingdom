#![feature(option_result_contains)]

use macroquad::window::{next_frame, Conf};

use crate::game::game;

pub mod asset_map;
pub mod astar;
pub mod conf;
pub mod game;
pub mod geometry;
pub mod map;
pub mod map_gen;
pub mod math;
pub mod objects;
pub mod util;

/// Config for the game
fn config() -> Conf {
    Conf {
        window_title: "Automated Kingdom".to_owned(),
        ..Default::default()
    }
}

#[macroquad::main(config)]
async fn main() {
    game().init().await;
    loop {
        game().update();
        game().draw();
        next_frame().await;
    }
}