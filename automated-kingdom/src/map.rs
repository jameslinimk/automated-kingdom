//! Contains map struct and related functions

use ak_server::types_game::{Texture, Tile};
use derive_new::new;
use macroquad::prelude::{is_mouse_button_down, uvec2, vec2, MouseButton, UVec2, Vec2, RED, WHITE};
use macroquad::shapes::draw_circle;
use macroquad::texture::{draw_texture, DrawTextureParams};
use macroquad::window::{screen_height, screen_width};
use rustc_hash::FxHashSet;

use crate::conf::SQUARE_SIZE;
use crate::game::game;
use crate::geometry::CollisionRect;
use crate::objects::buildings::BuildingTrait;
use crate::objects::ore_patch::{Ore, OrePatch};
use crate::objects::player::bottom_ui_height;
use crate::objects::worker::workers_iter;
use crate::texture_map::TextureMap;
use crate::util::{
    draw_rel_rectangle, draw_rel_texture_ex, mouse_pos, relative_zoom, screen_mouse_pos,
};
use crate::{hashset, hex, ternary};

/// Stores information about the map, such as walls, tiles, ores, etc. Player specific stuff, IE buildings, workers, etc. are stored in the [crate::objects::player::Player] struct
#[derive(PartialEq, Clone, Debug, new)]
pub(crate) struct Map {
    /// Stores the base map, which is the map without any player specific stuff and just walls
    #[new(value = "string_to_map(TEST_MAP).0")]
    pub(crate) base_map: Vec<Vec<Tile>>,

    /// Stores the tiles that that on the [Self::base_map] that are collidable, meaning workers can't walk on them
    #[new(value = "hashset![]")]
    pub(crate) tiles: FxHashSet<UVec2>,

    /// Stores ore patches around the map
    #[new(value = "vec![
        OrePatch::new(uvec2(5, 5), Ore::Gold, 1000)
    ]")]
    pub(crate) ores: Vec<OrePatch>,

    /// Stores the width of the [Self::base_map]
    #[new(value = "string_to_map(TEST_MAP).1")]
    pub(crate) width: usize,

    /// Stores the height of the [Self::base_map]
    #[new(value = "string_to_map(TEST_MAP).2")]
    pub(crate) height: usize,
}
impl Map {
    /// Returns a tile at a given position
    pub(crate) fn get(&self, pos: UVec2) -> Tile {
        self.base_map[pos.y as usize][pos.x as usize]
    }

    /// Draws the map and all the ores to the screen
    pub(crate) fn draw(&self) {
        for (y, row) in self.base_map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let world_pos = Map::pos_to_world(uvec2(x as u32, y as u32));
                match tile {
                    Tile::Wall => {
                        draw_texture(Texture::Wall.texture(), world_pos.x, world_pos.y, WHITE);
                    }
                    Tile::Air => {}
                }
            }
        }

        for ore in self.ores.iter() {
            ore.draw();
        }
    }

    /// Draws a minimap of the map to the screen, with the camera position, view indicator, and dots for ore patches, workers, etc.
    pub(crate) fn draw_minimap(&self) {
        let divisor = 10.0;
        let new_square_size = SQUARE_SIZE / divisor;
        let margin = 8.0;

        /* --------------------------------- Drawing -------------------------------- */
        // Border / background
        let width = self.width as f32 * new_square_size;
        let height = self.height as f32 * new_square_size;

        let border_width = 2.0;

        draw_rel_rectangle(
            border_width,
            border_width,
            width + margin * 2.0 - border_width,
            height + margin * 2.0 - border_width,
            hex!("#DCB579"),
        );
        let border_rect = CollisionRect::new_rel(
            border_width,
            border_width,
            width + margin * 2.0 - border_width,
            height + margin * 2.0 - border_width,
        );
        border_rect.draw_lines(relative_zoom(3.0), hex!("#A0793D"));

        /* ----------------------------------- Map ---------------------------------- */
        for (y, row) in self.base_map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                match tile {
                    Tile::Wall => {
                        draw_rel_texture_ex(
                            Texture::Wall.texture(),
                            margin + x as f32 * new_square_size,
                            margin + y as f32 * new_square_size,
                            DrawTextureParams {
                                dest_size: Some(vec2(new_square_size, new_square_size)),
                                ..Default::default()
                            },
                        );
                    }
                    Tile::Air => {}
                }
            }
        }

        /* ----------------------------- View indicator ----------------------------- */
        let cam_center = game().camera.camera.target;
        let cam_view = vec2(screen_width(), screen_height()) / game().camera.zoom;
        let cam_rect = CollisionRect::new_rel(
            margin + (cam_center.x - cam_view.x / 2.0) / divisor,
            margin + (cam_center.y - cam_view.y / 2.0) / divisor,
            cam_view.x / divisor,
            (cam_view.y - bottom_ui_height()) / divisor,
        );

        cam_rect.draw_lines(relative_zoom(2.0), RED);

        let person_dot_size = 6.0;
        let ore_dot_size = 8.0;

        // Workers
        for worker in workers_iter() {
            let p_color = game().main_player().color;
            let color = ternary!(worker.color == p_color, hex!("#00FF00"), hex!("#FF0000"));

            let center = worker.rect.center();
            let rect = CollisionRect::new_rel_center(
                center / divisor + margin,
                person_dot_size,
                person_dot_size,
            );

            if worker.color != p_color && !cam_rect.touches_point(&rect.center()) {
                continue;
            }

            rect.draw(color);
        }

        // Ores
        for ore in self.ores.iter() {
            let ore_rect = Map::pos_to_rect(ore.pos, ore.width, ore.height);
            let rect = CollisionRect::new_rel_center(
                ore_rect.center() / divisor + margin,
                ore_dot_size,
                ore_dot_size,
            );

            if !cam_rect.touches_rect(&rect) {
                continue;
            }

            rect.draw(ore.ore.color());
        }

        /* -------------------------------- Movement -------------------------------- */
        if border_rect.touches_point(&screen_mouse_pos()) && is_mouse_button_down(MouseButton::Left)
        {
            let new_cam_pos = (mouse_pos() - vec2(margin, margin / 2.0)) * divisor;
            draw_circle(cam_rect.center().x, cam_rect.center().y, 2.0, RED);
            game().camera.camera.target = new_cam_pos;
        }
    }

    /// Updates the [Map]'s tiles based on the current [OrePatch]'s and others
    pub(crate) fn update(&mut self) {
        for ore in self.ores.iter() {
            for x in ore.pos.x..ore.pos.x + ore.width {
                for y in ore.pos.y..ore.pos.y + ore.height {
                    self.tiles.insert(uvec2(x, y));
                }
            }
        }

        for player in game().players.iter() {
            for building in player.buildings.iter() {
                for x in building.pos().x..building.pos().x + building.size().0 {
                    for y in building.pos().y..building.pos().y + building.size().1 {
                        self.tiles.insert(uvec2(x, y));
                    }
                }
            }
        }
    }

    /// Updates the camera bounds based on the current base map size
    pub(crate) fn set_camera_bounds(&self) {
        game().camera.bounds = Some(vec2(
            self.width as f32 * SQUARE_SIZE,
            self.height as f32 * SQUARE_SIZE,
        ));
    }

    /// Converts a location on a [Map] to a [CollisionRect] with the given width and height
    pub(crate) fn pos_to_rect(pos: UVec2, width: u32, height: u32) -> CollisionRect {
        let world_pos = Map::pos_to_world(pos);
        CollisionRect::new_vec2(
            world_pos,
            width as f32 * SQUARE_SIZE,
            height as f32 * SQUARE_SIZE,
        )
    }

    /// Inverse of [Self::world_to_pos]. Converts a location on a [Map] to a world position
    pub(crate) fn pos_to_world(pos: UVec2) -> Vec2 {
        vec2(pos.x as f32 * SQUARE_SIZE, pos.y as f32 * SQUARE_SIZE)
    }

    /// Inverse of [Self::pos_to_world], converts a world position to a location on a [Map]
    pub(crate) fn world_to_pos(pos: Vec2) -> UVec2 {
        uvec2((pos.x / SQUARE_SIZE) as u32, (pos.y / SQUARE_SIZE) as u32)
    }
}

/// Converts a string of `#` and `.` to a [Map]. Returns `(map, width, height)`
fn string_to_map(string: &'static str) -> (Vec<Vec<Tile>>, usize, usize) {
    let mut map = vec![];
    for line in string.lines() {
        let mut row = vec![];
        for c in line.chars() {
            match c {
                '#' => row.push(Tile::Wall),
                '.' => row.push(Tile::Air),
                _ => panic!("Invalid character in map string '{}'", c),
            }
        }
        map.push(row);
    }
    let width = map[0].len();
    let height = map.len();
    (map, width, height)
}

const TEST_MAP: &str = "######################################################################################################################################################
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
#....................................................................................................................................................#
######################################################################################################################################################";
