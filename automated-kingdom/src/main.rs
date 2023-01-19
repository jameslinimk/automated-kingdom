#![feature(concat_idents)]

use std::thread;
use std::time::Duration;

use macroquad::time::{get_fps, get_frame_time};
use macroquad::window::{next_frame, Conf};

use crate::config::config;
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
    // std::env::set_var("RUST_BACKTRACE", "1");

    Game::preload();
    game().init();
    loop {
        println!("{:?}", get_fps());
        game().update();
        game().draw();
        next_frame().await;

        // limit fps
        let frame_time = get_frame_time();
        if frame_time < 1.0 / 60.0 {
            thread::sleep(Duration::from_secs_f32(1.0 / 60.0 - frame_time));
        }
    }
}
