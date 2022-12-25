use std::sync::Mutex;

use lazy_static::lazy_static;
use macroquad::prelude::ImageFormat;
use macroquad::texture::{FilterMode, Texture2D};
use rustc_hash::FxHashMap;

use crate::hashmap;

lazy_static! {
    static ref ASSET_MAP: Mutex<FxHashMap<&'static str, Texture2D>> = Mutex::from(hashmap! {});
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
