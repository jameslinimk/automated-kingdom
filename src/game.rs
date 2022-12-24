use derive_new::new;

use crate::asset_map::add_texture;
use crate::conf::SILVER_FONT;
use crate::map::Map;
use crate::objects::camera::Camera;
use crate::objects::player::Player;
use crate::objects::worker::workers_iter_mut;

static mut GAME: Option<Game> = None;

/// Returns the global [Game] object as a mutable reference
pub fn game() -> &'static mut Game {
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

    #[new(value = "Map::new()")]
    pub map: Map,
}
impl Game {
    pub async fn init(&mut self) {
        let _ = *SILVER_FONT;
        add_texture("wall", include_bytes!("../assets/sprites/wall.png"));
        self.map.update_camera_bounds();
    }

    pub fn update(&mut self) {
        self.players[self.main_player].update();
        self.camera.update();
        for worker in workers_iter_mut() {
            worker.update();
        }
    }

    pub fn draw(&mut self) {
        self.map.draw();
        self.players[self.main_player].draw();
        self.map.draw_minimap();
    }
}
