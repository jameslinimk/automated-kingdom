use derive_new::new;
use macroquad::prelude::{is_mouse_button_pressed, MouseButton};

use crate::astar::astar;
use crate::game::get_game;
use crate::map::world_to_loc;
use crate::objects::worker::Worker;
use crate::util::rel_mouse_pos;

#[derive(new)]
pub struct Player {
    #[new(value = "vec![Worker::new()]")]
    pub workers: Vec<Worker>,

    #[new(value = "None")]
    pub selected_worker: Option<u16>,
}
impl Player {
    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            for worker in self.workers.iter() {
                if worker.rect.touches_point(&rel_mouse_pos()) {
                    if self.selected_worker.contains(&worker.id) {
                        self.selected_worker = None;
                    } else {
                        self.selected_worker = Some(worker.id);
                    }
                    break;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            if let Some(id) = self.selected_worker {
                for worker in self.workers.iter_mut() {
                    if worker.id == id {
                        let path = astar(
                            &world_to_loc(&worker.rect.center()),
                            &world_to_loc(&rel_mouse_pos()),
                            &get_game().map,
                        );
                        worker.path = path;
                        break;
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        for worker in self.workers.iter() {
            if self.workers.contains(worker) {
                worker.draw(self.selected_worker.contains(&worker.id));
            }
        }
    }
}
