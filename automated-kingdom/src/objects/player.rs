use ak_server::types_game::{Color, ServerPlayer};
use derive_new::new;
use macroquad::prelude::{is_mouse_button_pressed, MouseButton, BLUE, GREEN, RED};
use macroquad::texture::DrawTextureParams;
use macroquad::window::{screen_height, screen_width};
use rustc_hash::FxHashMap;
use strum::IntoEnumIterator;

use crate::game::game;
use crate::map::Map;
use crate::objects::ore_patch::Ore;
use crate::objects::worker::Worker;
use crate::screen_size;
use crate::texture_map::TextureMap;
use crate::util::{abbreviate_number, draw_rel_rectangle, draw_rel_texture_ex, screen_mouse_pos};

pub(crate) fn bottom_ui_height() -> f32 {
    screen_size!(100.0, 150.0, 175.0)
}

#[derive(Clone, new)]
pub(crate) struct Player {
    #[new(
        value = "vec![Worker::new(Color::Blue), Worker::new(Color::Blue), Worker::new(Color::Blue), Worker::new(Color::Blue)]"
    )]
    pub(crate) workers: Vec<Worker>,

    #[new(value = "None")]
    pub(crate) selected_worker: Option<usize>,

    #[new(value = "Color::Blue")]
    pub(crate) color: Color,

    #[new(value = "0")]
    pub(crate) uuid: u64,

    #[new(value = "{
        let mut temp = FxHashMap::default();
        for ore in Ore::iter() {
            temp.insert(ore, 0);
        }
        temp
    }")]
    pub(crate) ores: FxHashMap<Ore, u32>,
}
impl Player {
    pub(crate) fn as_server(&self) -> ServerPlayer {
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

    pub(crate) fn update(&mut self) {
        if is_mouse_button_pressed(MouseButton::Left) {
            for (i, worker) in self.workers.iter().enumerate() {
                if worker.rect.touches_point(&screen_mouse_pos()) {
                    if self.selected_worker == Some(i) {
                        self.selected_worker = None;
                    } else {
                        self.selected_worker = Some(i);
                    }
                    break;
                }
            }
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            let pos = Map::world_to_pos(screen_mouse_pos());

            if let Some(worker) = self.selected_worker() {
                for (i, ore) in game().map.ores.iter().enumerate() {
                    if ore.as_rect().touches_point(&screen_mouse_pos()) {
                        worker.ore = Some(i);
                        worker.path = None;
                        return;
                    }
                }

                worker.set_path(pos);
                worker.ore = None;
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

    pub(crate) fn draw_ui(&mut self) {
        /* ------------------------------- Bottom part ------------------------------ */
        let general_info_width = screen_size!(128.0, 192.0, 256.0);

        // Info
        draw_rel_rectangle(
            0.0,
            screen_height() - bottom_ui_height(),
            general_info_width,
            bottom_ui_height(),
            RED,
        );

        for (i, (ore, amt)) in self.ores.iter().enumerate() {
            let margin = 4.0;
            let icon = ore.icon().texture();

            draw_rel_texture_ex(
                icon,
                margin,
                screen_height()
                    - (bottom_ui_height() - 32.0 * (i as f32) - margin * ((i + 1) as f32)),
                DrawTextureParams {
                    ..Default::default()
                },
            );

            let amt = abbreviate_number(*amt);
        }

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
    }

    pub(crate) fn draw(&mut self) {
        for (i, worker) in self.workers.iter().enumerate() {
            worker.draw(self.selected_worker == Some(i));
        }
        self.draw_ui();
    }
}
