use ak_server::types_game::{Color, ServerPlayer};
use derive_new::new;
use macroquad::prelude::{is_mouse_button_pressed, MouseButton, BLUE, GREEN, RED};
use macroquad::window::{screen_height, screen_width};

use crate::map::world_to_pos;
use crate::objects::worker::Worker;
use crate::screen_size;
use crate::util::{draw_rel_rectangle, rel_mouse_pos};

pub fn bottom_ui_height() -> f32 {
    screen_size!(100.0, 150.0, 175.0)
}

#[derive(new)]
pub struct Player {
    #[new(value = "vec![Worker::new()]")]
    pub workers: Vec<Worker>,

    #[new(value = "None")]
    pub selected_worker: Option<usize>,

    #[new(value = "Color::Blue")]
    pub color: Color,

    #[new(value = "0")]
    pub uuid: u64,
}
impl Player {
    pub fn as_server(&self) -> ServerPlayer {
        ServerPlayer {
            uuid: self.uuid,
            ping: 0,
            workers: self
                .workers
                .iter()
                .map(|worker| worker.as_server())
                .collect(),
            color: self.color,
        }
    }

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
                worker.path_to(world_to_pos(rel_mouse_pos()))
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
        let general_info_width = screen_size!(128.0, 192.0, 256.0);

        draw_rel_rectangle(
            0.0,
            screen_height() - bottom_ui_height(),
            general_info_width,
            bottom_ui_height(),
            RED,
        );

        // Selected worker image
        let selected_worker_width = bottom_ui_height();
        draw_rel_rectangle(
            general_info_width,
            screen_height() - bottom_ui_height(),
            selected_worker_width,
            bottom_ui_height(),
            BLUE,
        );

        // Selected worker info and commands
        draw_rel_rectangle(
            general_info_width + selected_worker_width,
            screen_height() - bottom_ui_height(),
            screen_width() - general_info_width - selected_worker_width,
            bottom_ui_height(),
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
