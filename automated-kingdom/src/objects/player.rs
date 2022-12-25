use derive_new::new;
use macroquad::prelude::{is_mouse_button_pressed, MouseButton, BLUE, GREEN, RED};
use macroquad::window::{screen_height, screen_width};

use crate::astar::astar;
use crate::game::game;
use crate::map::world_to_pos;
use crate::objects::worker::Worker;
use crate::util::{draw_rel_rectangle, rel_mouse_pos};

pub const BOTTOM_UI_HEIGHT: f32 = 64.0;

#[derive(new)]
pub struct Player {
    #[new(value = "vec![Worker::new()]")]
    pub workers: Vec<Worker>,

    #[new(value = "None")]
    pub selected_worker: Option<usize>,
}
impl Player {
    pub fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, worker) in self.workers.iter().enumerate() {
                if worker.rect.touches_point(&rel_mouse_pos()) {
                    if self.selected_worker.contains(&i) {
                        self.selected_worker = None;
                    } else {
                        self.selected_worker = Some(i);
                    }
                    break;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            if let Some(worker) = self.selected_worker() {
                let path = astar(
                    &world_to_pos(&worker.rect.center()),
                    &world_to_pos(&rel_mouse_pos()),
                    &game().map,
                );
                worker.path = path;
            }
        }
    }

    fn selected_worker(&mut self) -> Option<&mut Worker> {
        if let Some(index) = self.selected_worker {
            let worker = &mut self.workers[index];
            return Some(worker);
        }
        None
    }

    pub fn draw_ui(&mut self) {
        /* ------------------------------- Bottom part ------------------------------ */
        // General info
        let general_info_width = 256.0;
        draw_rel_rectangle(
            0.0,
            screen_height() - BOTTOM_UI_HEIGHT,
            general_info_width,
            BOTTOM_UI_HEIGHT,
            RED,
        );

        // Selected worker image
        let selected_worker_width = BOTTOM_UI_HEIGHT;
        draw_rel_rectangle(
            general_info_width,
            screen_height() - BOTTOM_UI_HEIGHT,
            selected_worker_width,
            BOTTOM_UI_HEIGHT,
            BLUE,
        );

        // Selected worker info and commands
        draw_rel_rectangle(
            general_info_width + selected_worker_width,
            screen_height() - BOTTOM_UI_HEIGHT,
            screen_width() - general_info_width - selected_worker_width,
            BOTTOM_UI_HEIGHT,
            GREEN,
        );

        // if let Some(worker) = self.get_selected_worker() {
        //     draw_rectangle(rx(0.0), ry(0.0), 10.0, 10.0, RED);
        // }
    }

    pub fn draw(&mut self) {
        for (i, worker) in self.workers.iter().enumerate() {
            worker.draw(self.selected_worker.contains(&i));
        }
        self.draw_ui();
    }
}
