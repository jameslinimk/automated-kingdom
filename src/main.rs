use macroquad::window::{next_frame, Conf};

use crate::game::get_game;

pub mod game;
pub mod math;
pub mod objects;

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
        next_frame().await;
    }
}
