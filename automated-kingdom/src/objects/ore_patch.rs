use ak_server::types_game::Texture;
use derive_new::new;
use macroquad::prelude::{Color, UVec2, GOLD, RED, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::time::get_time;
use rustc_hash::FxHashMap;
use strum_macros::EnumIter;

use crate::geometry::CollisionRect;
use crate::hashmap;
use crate::map::Map;
use crate::objects::worker::IdType;
use crate::texture_map::TextureMap;
use crate::util::draw_texture_center;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, EnumIter)]
pub enum Ore {
    Gold,
}
impl Ore {
    /// Color of the ore patch, used for the minimap
    pub fn color(&self) -> Color {
        match self {
            Ore::Gold => GOLD,
        }
    }

    /// Texture of the ore patch
    pub fn texture(&self) -> Texture {
        match self {
            Ore::Gold => Texture::GoldPatch,
        }
    }

    /// Icon of the ore
    pub fn icon(&self) -> Texture {
        match self {
            Ore::Gold => Texture::GoldIcon,
        }
    }

    /// Cooldown of the ore patch, in seconds
    pub fn cooldown(&self) -> f64 {
        match self {
            Ore::Gold => 0.5,
        }
    }

    /// Size of the ore patch in tiles
    pub fn size(&self) -> (u32, u32) {
        match self {
            Ore::Gold => (4, 4),
        }
    }
}

/// Represents an ore patch, which is a collection of ore in a certain area that can be mined
#[derive(Debug, PartialEq, Clone, new)]
pub struct OrePatch {
    /// Top-left in the [Map]
    pub pos: UVec2,
    /// Width of the patch on the [Map]
    #[new(value = "ore.size().0")]
    pub width: u32,
    /// Height of the patch on the [Map]
    #[new(value = "ore.size().1")]
    pub height: u32,

    /// What ore is in this patch
    pub ore: Ore,

    /// Max capacity of the patch
    pub max: u32,

    /// How much ore is left in the patch
    #[new(value = "max")]
    pub remaining: u32,

    #[new(value = "hashmap! {}")]
    pub mine_cooldowns: FxHashMap<IdType, f64>,
}
impl OrePatch {
    pub fn as_rect(&self) -> CollisionRect {
        Map::pos_to_rect(self.pos, self.width, self.height)
    }

    pub fn draw(&self) {
        let rect = self.as_rect();

        rect.draw(RED);
        draw_texture_center(
            self.ore.texture().texture(),
            rect.center().x,
            rect.center().y,
        );

        // Remaining bar
        let radio = self.remaining as f32 / self.max as f32;
        let w = rect.width;
        draw_rectangle(rect.left(), rect.top() - 10.0, w, 10.0, WHITE);
        draw_rectangle(rect.left(), rect.top() - 10.0, w * radio, 10.0, RED);
    }

    pub fn mine(&mut self, id: IdType) -> u32 {
        if let Some(last_mined) = self.mine_cooldowns.get(&id) {
            if get_time() - last_mined < self.ore.cooldown() {
                return 0;
            }
        }

        if self.remaining > 0 {
            self.remaining -= 1;
            self.mine_cooldowns.insert(id, get_time());
            1
        } else {
            0
        }
    }

    pub fn time_left(&self, id: IdType) -> f32 {
        (if let Some(last_mined) = self.mine_cooldowns.get(&id) {
            self.ore.cooldown() - (get_time() - last_mined)
        } else {
            0.0
        }) as f32
    }
}
