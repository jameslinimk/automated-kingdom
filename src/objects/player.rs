use derive_new::new;
use macroquad::prelude::{is_mouse_button_pressed, MouseButton, BLUE, GREEN, RED};
use macroquad::window::{screen_height, screen_width};

use crate::astar::astar;
use crate::game::game;
use crate::map::world_to_loc;
use crate::objects::worker::Worker;
use crate::util::{draw_rel_rectangle, rel_mouse_pos};

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
                    &world_to_loc(&worker.rect.center()),
                    &world_to_loc(&rel_mouse_pos()),
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
        let bottom_height = 64.0;
        let info_width = 256.0;

        // General info
        draw_rel_rectangle(
            0.0,
            screen_height() - bottom_height,
            info_width,
            bottom_height,
            RED,
        );

        // Selected worker image
        draw_rel_rectangle(
            info_width,
            screen_height() - bottom_height,
            bottom_height,
            bottom_height,
            BLUE,
        );

        // Selected worker info and commands
        draw_rel_rectangle(
            info_width + bottom_height,
            screen_height() - bottom_height,
            screen_width() - info_width,
            bottom_height,
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
