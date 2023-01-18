use ak_server::types_game::Texture;
use derive_new::new;
use macroquad::prelude::{Vec2, RED};

use crate::geometry::CollisionRect;
use crate::texture_map::TextureMap;
use crate::util::draw_texture_center;

#[derive(Clone, Copy)]
pub enum Ore {
    Gold,
}

#[derive(Clone, Copy, new)]
pub struct OrePatch {
    rect: CollisionRect,
    texture: Texture,
    pub ore: Ore,
	pub max: u32,
	#[new(value = "max")]
	pub remaining: u32,
}
impl OrePatch {
    pub fn new_gold(top_left: Vec2, max: u32) -> OrePatch {
        OrePatch::new(
            CollisionRect::new_vec2(top_left, 128.0, 128.0),
            Texture::GoldPatch,
            Ore::Gold,
			max
        )
    }

    pub fn draw(&self) {
        self.rect.draw(RED);
        draw_texture_center(
            self.texture.texture(),
            self.rect.center().x,
            self.rect.center().y,
        );
    }
}
