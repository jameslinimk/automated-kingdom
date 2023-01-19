use ak_server::types_game::Texture;
use derive_new::new;
use macroquad::prelude::{Color, UVec2, GOLD, RED, WHITE};
use macroquad::shapes::draw_rectangle;
use macroquad::texture::Texture2D;

use crate::map::Map;
use crate::texture_map::TextureMap;
use crate::util::draw_texture_center;

#[derive(Hash, Debug, PartialEq, Eq, Clone, Copy)]
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

    /// Size of the ore patch in tiles
    pub fn size(&self) -> (u32, u32) {
        match self {
            Ore::Gold => (4, 4),
        }
    }
}

/// Represents an ore patch, which is a collection of ore in a certain area that can be mined
#[derive(Debug, PartialEq, Clone, Copy, new)]
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
    #[new(value = "ore.texture().texture()")]
    texture: Texture2D,

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
        draw_texture_center(self.texture, rect.center().x, rect.center().y);

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
