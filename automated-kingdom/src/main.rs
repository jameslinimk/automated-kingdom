#![feature(concat_idents)]

use macroquad::window::{next_frame, Conf};

use crate::config::config;
use crate::game::{game, Game};

pub(crate) mod astar;
pub(crate) mod conf;
pub(crate) mod config;
pub(crate) mod game;
pub(crate) mod geometry;
pub(crate) mod map;
pub(crate) mod map_gen;
pub(crate) mod math;
pub(crate) mod objects;
pub(crate) mod spritesheet;
pub(crate) mod texture_map;
pub(crate) mod util;

/// Base config for the game
fn base_cfg() -> Conf {
    let conf = config();
    Conf {
        window_title: "Automated Kingdom".to_owned(),
        window_resizable: true,
        window_width: conf.window_width,
        window_height: conf.window_height,
        ..Default::default()
    }
}

#[cfg(not(windows))]
fn cfg() -> Conf {
    base_cfg()
}

#[cfg(windows)]
fn cfg() -> Conf {
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

#[macroquad::main(cfg)]
async fn main() {
    Game::preload();
    game().init();

    loop {
        game().update();
        game().draw();
        next_frame().await;
    }
}
