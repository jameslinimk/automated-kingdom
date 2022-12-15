use derive_new::new;

use crate::objects::camera::Camera;
use crate::objects::player::Player;
use crate::objects::worker::get_workers;

static mut GAME: Option<Game> = None;

/// Returns the global [Game] object as a mutable reference
pub fn get_game() -> &'static mut Game {
    unsafe {
        if GAME.is_none() {
            GAME = Some(Game::new());
        }
        GAME.as_mut().unwrap()
    }
}

#[derive(new)]
pub struct Game {
    #[new(value = "vec![Player::new()]")]
    pub players: Vec<Player>,

    #[new(value = "0")]
    pub main_player: usize,

    #[new(value = "Camera::new()")]
    pub camera: Camera,
}
impl Game {
    pub fn update(&mut self) {
        self.players[self.main_player].update();
        self.camera.update();
        for worker in get_workers().values_mut() {
            worker.update();
        }
    }

    pub fn draw(&mut self) {
        self.players[self.main_player].draw();
    }
}
