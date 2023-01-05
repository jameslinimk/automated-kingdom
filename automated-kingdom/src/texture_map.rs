use std::sync::Mutex;

use ak_server::types_game::Texture;
use lazy_static::lazy_static;
use macroquad::prelude::ImageFormat;
use macroquad::texture::{FilterMode, Texture2D};
use rustc_hash::FxHashMap;

use crate::hashmap;

lazy_static! {
    /// A map of all textures loaded into the game
    static ref TEXTURE_MAP: Mutex<FxHashMap<Texture, Texture2D>> = Mutex::from(hashmap! {});
}

/// A trait for getting a texture from the [TEXTURE_MAP]
pub trait TextureMap {
    fn texture(&self) -> Texture2D;
}
impl TextureMap for Texture {
    /// Gets the given texture from the [TEXTURE_MAP]
    fn texture(&self) -> Texture2D {
        *TEXTURE_MAP.lock().unwrap().get(self).unwrap()
    }
}

/// Adds a texture to the texture map
pub fn load_texture(name: Texture, bytes: &'static [u8]) {
    let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    texture.set_filter(FilterMode::Nearest);
    TEXTURE_MAP.lock().unwrap().insert(name, texture);
}
