use derive_new::new;
use macroquad::prelude::{is_mouse_button_pressed, MouseButton};

use crate::objects::worker::{get_workers, get_workers_mut, Worker};
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
            for worker in get_workers() {
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
            if let Some(worker) = self.selected_worker {
                for w in get_workers_mut() {
                    if w.id == worker {
                        w.path = Some(vec![rel_mouse_pos()]);
                        break;
                    }
                }
            }
        }
    }

    pub fn draw(&mut self) {
        for worker in get_workers() {
            if self.workers.contains(worker) {
                worker.draw(self.selected_worker.contains(&worker.id));
            }
        }
    }
}
