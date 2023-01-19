#![feature(concat_idents)]

use macroquad::window::{next_frame, Conf};

use crate::game::{game, Game};

pub mod astar;
pub mod conf;
pub mod config;
pub mod game;
pub mod geometry;
pub mod map;
pub mod map_gen;
pub mod math;
pub mod objects;
pub mod spritesheet;
pub mod texture_map;
pub mod util;

/// Base config for the game
fn base_cfg() -> Conf {
    Conf {
        window_title: "Automated Kingdom".to_owned(),
        window_resizable: true,
        window_width: 1280,
        window_height: 1280,
        ..Default::default()
    }
}

#[cfg(not(windows))]
fn config() -> Conf {
    base_cfg()
}

#[cfg(windows)]
fn config() -> Conf {
    use std::io::Cursor;

    use image::io::Reader;
    use macroquad::miniquad::conf::Icon;

    macro_rules! image {
        ($path: expr) => {
            Reader::new(Cursor::new(include_bytes!($path)))
                .with_guessed_format()
                .unwrap()
                .decode()
                .unwrap()
                .to_rgba8()
                .to_vec()
                .try_into()
                .unwrap()
        };
    }

    Conf {
        icon: Some(Icon {
            small: image!("../assets/icons/icon_16.png"),
            medium: image!("../assets/icons/icon_32.png"),
            big: image!("../assets/icons/icon_64.png"),
        }),
        ..base_cfg()
    }
}

#[macroquad::main(config)]
async fn main() {
    // std::env::set_var("RUST_BACKTRACE", "1");

    Game::preload();
    game().init();
    loop {
        game().update();
        game().draw();
        next_frame().await;
    }
}
