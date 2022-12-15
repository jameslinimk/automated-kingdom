use derive_new::new;
use macroquad::prelude::{is_mouse_button_down, MouseButton};
use macroquad::time::get_frame_time;
use rustc_hash::FxHashSet;

use crate::hashset;
use crate::objects::worker::{get_workers, Worker};
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
        println!("{}", get_frame_time());

        if is_mouse_button_down(MouseButton::Left) {
            for worker in get_workers().values() {
                if worker.rect.touches_point(&rel_mouse_pos()) {
                    self.selected_worker = Some(worker.id);
                    break;
                }
            }
        }
    }

    pub fn draw(&mut self) {
        for worker in get_workers().values() {
            if self.workers.contains(&worker.id) {
                worker.draw(self.selected_worker.contains(&worker.id));
            }
        }
    }
}
