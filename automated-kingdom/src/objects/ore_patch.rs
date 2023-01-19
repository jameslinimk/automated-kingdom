use ak_server::types_game::Texture;
use derive_new::new;
use lazy_static::lazy_static;
use macroquad::prelude::{Color, UVec2, GOLD, RED, WHITE};
use macroquad::shapes::draw_rectangle;
use rustc_hash::FxHashMap;

use crate::hashmap;
use crate::map::Map;
use crate::texture_map::TextureMap;
use crate::util::draw_texture_center;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
pub enum Ore {
    Gold,
}
impl Ore {
    pub fn color(&self) -> Color {
        match self {
            Ore::Gold => GOLD,
        }
    }

    pub fn texture(&self) -> Texture {
        *ORE_TEXTURE_MAP.get(self).unwrap()
    }
}

lazy_static! {
    static ref ORE_TEXTURE_MAP: FxHashMap<Ore, Texture> = hashmap! {
        Ore::Gold => Texture::GoldPatch
    };
}

/// Represents an ore patch, which is a collection of ore in a certain area that can be mined
#[derive(Debug, PartialEq, Clone, Copy, new)]
pub struct OrePatch {
    /// Top-left in the [Map]
    pub pos: UVec2,
    /// Width of the patch on the [Map]
    pub width: u32,
    /// Height of the patch on the [Map]
    pub height: u32,

    /// What ore is in this patch
    pub ore: Ore,
    #[new(value = "ore.texture()")]
    texture: Texture,

    /// Max capacity of the patch
    pub max: u32,

    /// How much ore is left in the patch
    #[new(value = "max")]
    pub remaining: u32,
}
impl OrePatch {
    pub fn draw(&self) {
        let rect = Map::pos_to_rect(self.pos, self.width, self.height);

        rect.draw(RED);
        draw_texture_center(self.texture.texture(), rect.center().x, rect.center().y);

        // Remaining bar
        let radio = self.remaining as f32 / self.max as f32;
        let w = rect.width;
        draw_rectangle(rect.left(), rect.top() - 10.0, w, 10.0, WHITE);
        draw_rectangle(rect.left(), rect.top() - 10.0, w * radio, 10.0, RED);
    }

    pub fn mine(&mut self) -> (u32, Ore) {
        if self.remaining > 0 {
            self.remaining -= 1;
            (1, self.ore)
        } else {
            (0, self.ore)
        }
    }
}
