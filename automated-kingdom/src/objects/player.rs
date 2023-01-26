use ak_server::types_game::{Color, ServerPlayer};
use derive_new::new;
use macroquad::prelude::{
    is_mouse_button_pressed, uvec2, vec2, MouseButton, UVec2, BLUE, GREEN, RED, WHITE,
};
use macroquad::text::measure_text;
use macroquad::texture::{draw_texture_ex, DrawTextureParams};
use macroquad::window::{screen_height, screen_width};
use rustc_hash::FxHashMap;
use strum::IntoEnumIterator;

use crate::conf::SILVER_FONT;
use crate::game::game;
use crate::geometry::CollisionRect;
use crate::map::Map;
use crate::objects::buildings::Building;
use crate::objects::ore_patch::Ore;
use crate::objects::worker::Worker;
use crate::texture_map::TextureMap;
use crate::util::{
    abbreviate_number, draw_rel_rectangle, draw_rel_text_top_left, draw_rel_texture_ex,
    draw_texture_center, relative_zoom_vec2, screen_mouse_pos, UVec2SaturatedSub,
};
use crate::{hex, screen_size};

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

    #[new(value = "vec![]")]
    pub(crate) buildings: Vec<Building>,

    /// Selected building to place
    #[new(value = "None")]
    pub(crate) selected_new_building: Option<Building>,

    #[new(value = "None")]
    selected_new_building_pos: Option<UVec2>,
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

    /// Updates controlling workers
    pub(crate) fn update_workers(&mut self) {
        // Selecting workers
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

        // Right click action
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

    /// Updates placing buildings
    pub(crate) fn update_placing(&mut self) {
        if let Some(selected) = self.selected_new_building {
            let (width, height) = selected.size();

            let mp = Map::world_to_pos(screen_mouse_pos());

            let padding = 2;
            let padding_rect = Map::pos_to_rect(
                mp.saturated_sub(uvec2(padding, padding) * 2),
                width + padding * 2,
                height + padding * 2,
            );
            padding_rect.draw(hex!("#ff0000", 128));

            let selected_rect =
                Map::center_pos_to_rect(padding_rect.center().as_uvec2(), width, height);

            let mp = selected_rect.center();
            draw_texture_center(selected.texture().texture(), mp.x, mp.y);

            if is_mouse_button_pressed(MouseButton::Right) {
                self.selected_new_building = None;
                self.selected_new_building_pos = None;
                return;
            }

            if is_mouse_button_pressed(MouseButton::Left) {}
        }
    }

    pub(crate) fn update(&mut self) {
        self.update_workers();
        self.update_placing();
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
        let margin = 4.0;

        // Info
        draw_rel_rectangle(
            0.0,
            screen_height() - bottom_ui_height(),
            general_info_width,
            bottom_ui_height(),
            RED,
        );

        for (i, (ore, amt)) in self.ores.iter().enumerate() {
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

            let font_size = 32;
            let amt = format!("x{}", abbreviate_number(*amt));
            let measurements = measure_text(&amt, Some(*SILVER_FONT), font_size, 1.0);
            draw_rel_text_top_left(
                &amt,
                32.0 + margin * 2.0,
                screen_height()
                    - (bottom_ui_height() - 32.0 * (i as f32) - margin * ((i + 1) as f32))
                    + measurements.height / 2.0,
                font_size as f32,
                WHITE,
            );
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
        let x = general_info_width + selected_worker_width;
        let y = screen_height() - bottom_ui_height();
        draw_rel_rectangle(
            x,
            y,
            screen_width() - general_info_width - selected_worker_width,
            bottom_ui_height(),
            GREEN,
        );

        for building in Building::iter() {
            let texture = building.icon().texture();
            let rect =
                CollisionRect::new_rel(x + margin, y + margin, texture.width(), texture.height());
            draw_texture_ex(
                texture,
                rect.left(),
                rect.top(),
                WHITE,
                DrawTextureParams {
                    dest_size: Some(relative_zoom_vec2(vec2(texture.width(), texture.height()))),
                    ..Default::default()
                },
            );

            if is_mouse_button_pressed(MouseButton::Left) && rect.touches_point(&screen_mouse_pos())
            {
                if self.selected_new_building == Some(building) {
                    self.selected_new_building = None;
                    self.selected_new_building_pos = None;
                    return;
                }

                self.selected_new_building = Some(building);
                self.selected_new_building_pos = Some(Map::world_to_pos(screen_mouse_pos()));
            }
        }
    }

    pub(crate) fn draw(&mut self) {
        for (i, worker) in self.workers.iter().enumerate() {
            worker.draw(self.selected_worker == Some(i));
        }
        self.draw_ui();
    }
}
