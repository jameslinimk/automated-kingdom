//! Module for loading and storing textures, as well as some utility traits for drawing sprites

use std::sync::Mutex;

use ak_server::types_game::{Sprite, Texture};
use lazy_static::lazy_static;
use macroquad::prelude::{ImageFormat, Rect, WHITE};
use macroquad::texture::{draw_texture, draw_texture_ex, DrawTextureParams, FilterMode, Texture2D};
use rustc_hash::FxHashMap;

use crate::hashmap;

lazy_static! {
    /// A map of all textures loaded into the game
    static ref TEXTURE_MAP: Mutex<FxHashMap<Texture, Texture2D>> = Mutex::from(hashmap! {});
}

/// Adds a texture to the texture map
pub(crate) fn load_texture(name: Texture, bytes: &'static [u8]) {
    let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    texture.set_filter(FilterMode::Nearest);
    TEXTURE_MAP.lock().unwrap().insert(name, texture);
}

/// trait for getting a texture from the [static@TEXTURE_MAP]
pub(crate) trait TextureMap {
    /// Gets the given texture from the [static@TEXTURE_MAP]
    fn texture(&self) -> Texture2D;
}
impl TextureMap for Texture {
    fn texture(&self) -> Texture2D {
        *TEXTURE_MAP
            .lock()
            .unwrap()
            .get(self)
            .unwrap_or_else(|| panic!("Texture not found for \"{self:?}\""))
    }
}

/// Trait for drawing [Sprite]'s sent from server in the game
pub(crate) trait DrawSprite {
    /// Draw the given sprite at the given top-left position
    fn draw(&self, x: f32, y: f32);
}
impl DrawSprite for Sprite {
    fn draw(&self, x: f32, y: f32) {
        match self {
            Sprite::Sprite(texture) => {
                let texture = texture.texture();
                draw_texture(texture, x, y, WHITE);
            }

            Sprite::SpriteSheet {
                texture,
                frames,
                current_frame,
            } => {
                let base_texture = texture.texture();
                let w = base_texture.width() / *frames as f32;
                let h = base_texture.height();

                let rect = Rect::new(*current_frame as f32 * w, 0.0, w, h);
                draw_texture_ex(
                    base_texture,
                    x,
                    y,
                    WHITE,
                    DrawTextureParams {
                        source: Some(rect),
                        ..Default::default()
                    },
                );
            }
        }
    }
}
