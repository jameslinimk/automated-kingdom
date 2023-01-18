#![feature(concat_idents)]

use std::env;

use macroquad::window::{next_frame, Conf};

use crate::game::{game, Game};

pub mod astar;
pub mod conf;
pub mod game;
pub mod geometry;
pub mod map;
pub mod map_gen;
pub mod math;
pub mod objects;
pub mod spritesheet;
pub mod texture_map;
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
    env::set_var("RUST_BACKTRACE", "1");

    Game::preload();
    game().init();
    loop {
        game().update();
        game().draw();
        next_frame().await;
    }
}
