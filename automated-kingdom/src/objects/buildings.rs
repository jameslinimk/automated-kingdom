use ak_server::types_game::Texture;
use enum_assoc::Assoc;
use enum_dispatch::enum_dispatch;
use macroquad::prelude::UVec2;
use rustc_hash::FxHashMap;
use strum_macros::EnumIter;

use crate::conf::SQUARE_SIZE;
use crate::hashmap;
use crate::objects::ore_patch::Ore;
use crate::texture_map::TextureMap;

#[enum_dispatch]
pub(crate) trait BuildingTrait {
    fn pos(&self) -> UVec2;
}

#[derive(Clone, Copy, Default)]
pub(crate) struct House {
    pos: UVec2,
}
impl BuildingTrait for House {
    fn pos(&self) -> UVec2 {
        self.pos
    }
}

#[enum_dispatch(BuildingTrait)]
#[derive(Clone, Copy, Assoc, EnumIter)]
#[func(pub(crate) fn texture(&self) -> Texture)]
#[func(pub(crate) fn icon(&self) -> Texture)]
#[func(pub(crate) fn cost(&self) -> FxHashMap<Ore, u32>)]
pub(crate) enum Building {
    #[assoc(texture = Texture::House)]
    #[assoc(icon = Texture::HouseIcon)]
    #[assoc(cost = hashmap! { Ore::Gold => 10 })]
    House,
}
impl Building {
    /// Size of the ore building in tiles
    pub(crate) fn size(&self) -> (u32, u32) {
        let texture = self.texture().texture();
        (
            (texture.width() / SQUARE_SIZE) as u32,
            (texture.height() / SQUARE_SIZE) as u32,
        )
    }
}
