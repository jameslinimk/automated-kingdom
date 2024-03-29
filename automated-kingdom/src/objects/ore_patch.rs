use ak_server::types_game::Texture;
use derive_new::new;
use enum_assoc::Assoc;
use macroquad::prelude::{Color, UVec2, GOLD, RED, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::time::get_time;
use rustc_hash::FxHashMap;
use strum_macros::EnumIter;

use crate::conf::SQUARE_SIZE;
use crate::geometry::CollisionRect;
use crate::hashmap;
use crate::map::Map;
use crate::objects::worker::IdType;
use crate::texture_map::TextureMap;
use crate::util::draw_texture_center;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy, EnumIter, Assoc)]
#[func(pub(crate) fn color(&self) -> Color)]
#[func(pub(crate) fn texture(&self) -> Texture)]
#[func(pub(crate) fn icon(&self) -> Texture)]
#[func(pub(crate) fn cooldown(&self) -> f64)]
pub(crate) enum Ore {
    #[assoc(color = GOLD)]
    #[assoc(texture = Texture::GoldPatch)]
    #[assoc(icon = Texture::GoldIcon)]
    #[assoc(cooldown = 0.5)]
    Gold,
}
impl Ore {
    /// Size of the ore patch in tiles
    pub(crate) fn size(&self) -> (u32, u32) {
        let texture = self.texture().texture();
        (
            (texture.width() / SQUARE_SIZE) as u32,
            (texture.height() / SQUARE_SIZE) as u32,
        )
    }
}

/// Represents an ore patch, which is a collection of ore in a certain area that can be mined
#[derive(Debug, PartialEq, Clone, new)]
pub(crate) struct OrePatch {
    /// Top-left in the [Map]
    pub(crate) pos: UVec2,
    /// Width of the patch on the [Map]
    #[new(value = "ore.size().0")]
    pub(crate) width: u32,
    /// Height of the patch on the [Map]
    #[new(value = "ore.size().1")]
    pub(crate) height: u32,

    /// What ore is in this patch
    pub(crate) ore: Ore,

    /// Max capacity of the patch
    pub(crate) max: u32,

    /// How much ore is left in the patch
    #[new(value = "max")]
    pub(crate) remaining: u32,

    #[new(value = "hashmap! {}")]
    pub(crate) mine_cooldowns: FxHashMap<IdType, f64>,
}
impl OrePatch {
    pub(crate) fn as_rect(&self) -> CollisionRect {
        Map::pos_to_rect(self.pos, self.width, self.height)
    }

    pub(crate) fn draw(&self) {
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

    pub(crate) fn mine(&mut self, id: IdType) -> u32 {
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

    pub(crate) fn time_left(&self, id: IdType) -> f32 {
        (if let Some(last_mined) = self.mine_cooldowns.get(&id) {
            self.ore.cooldown() - (get_time() - last_mined)
        } else {
            0.0
        }) as f32
    }
}
