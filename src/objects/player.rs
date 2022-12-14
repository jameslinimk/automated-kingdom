use derive_new::new;
use macroquad::prelude::{is_mouse_button_down, vec2, MouseButton};

use crate::get_worker;
use crate::objects::worker::{Worker, WORKERS};

#[derive(new)]
pub struct Player {
    #[new(value = "vec![Worker::new_add()]")]
    pub workers: Vec<u16>,

    #[new(value = "None")]
    pub selected_worker: Option<u16>,
}
impl Player {
    pub fn update(&mut self) {
        get_worker!(worker, self.workers[0]);
        worker.path = Some(vec![vec2(1000.0, 1000.0)]);

        if is_mouse_button_down(MouseButton::Left) {
            for worker in WORKERS.lock().unwrap().values() {}
        }
    }
}
