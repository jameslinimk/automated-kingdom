use std::sync::Mutex;

use lazy_static::lazy_static;
use macroquad::prelude::ImageFormat;
use macroquad::texture::{FilterMode, Texture2D};
use rustc_hash::FxHashMap;

use crate::hashmap;
use crate::spritesheet::SpriteSheet;

lazy_static! {
    static ref ASSET_MAP: Mutex<FxHashMap<&'static str, Texture2D>> = Mutex::from(hashmap! {});
    static ref SPRITESHEET_MAP: Mutex<FxHashMap<&'static str, SpriteSheet>> =
        Mutex::from(hashmap! {});
}

/// Gets a texture from the asset map
pub fn get_texture(name: &str) -> Texture2D {
    *ASSET_MAP.lock().unwrap().get(name).unwrap()
}

/// Adds a texture to the asset map
pub fn add_texture(name: &'static str, bytes: &'static [u8]) {
    let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    texture.set_filter(FilterMode::Nearest);
    ASSET_MAP.lock().unwrap().insert(name, texture);
}

/// Gets a spritesheet from the asset map
pub fn get_spritesheet(name: &str) -> SpriteSheet {
    *SPRITESHEET_MAP.lock().unwrap().get(name).unwrap()
}

/// Adds a spritesheet to the asset map
pub fn add_spritesheet(name: &'static str, bytes: &'static [u8], width: u16, frame_duration: f32) {
    let texture = Texture2D::from_file_with_format(bytes, Some(ImageFormat::Png));
    texture.set_filter(FilterMode::Nearest);
    SPRITESHEET_MAP
        .lock()
        .unwrap()
        .insert(name, SpriteSheet::new(texture, width, frame_duration));
}
