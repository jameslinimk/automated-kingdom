use std::sync::atomic::{AtomicU16, Ordering};

use ak_server::types_game::{Color as ServerColor, ServerWorker, Texture, Tile};
use derive_new::new;
use macroquad::color_u8;
use macroquad::prelude::{uvec2, vec2, Color, UVec2, Vec2, RED, WHITE};
use macroquad::shapes::{draw_line, draw_rectangle};
use macroquad::texture::DrawTextureParams;
use macroquad::time::get_frame_time;
use rustc_hash::FxHashMap;

use crate::astar::{astar, path_time};
use crate::conf::SQUARE_SIZE;
use crate::game::game;
use crate::geometry::CollisionRect;
use crate::map::Map;
use crate::math::{angle, distance, opposite_angle, project, u_distance};
use crate::spritesheet::SpriteSheet;
use crate::texture_map::TextureMap;
use crate::util::{draw_text_center, draw_texture_center_ex, FloatSignum};
use crate::{derive_id_eq, hashmap};

pub(crate) static GLOBAL_ID: AtomicU16 = AtomicU16::new(0);
pub(crate) type IdType = u16;

/// Returns a new id for the global id system
#[inline]
pub(crate) fn new_id() -> IdType {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed)
}

/// Returns an iterator over all workers in the game
#[inline]
pub(crate) fn workers_iter() -> impl Iterator<Item = &'static Worker> {
    game().players.iter().flat_map(|p| p.workers.iter())
}

/// Returns a mutable iterator over all workers in the game
#[inline]
pub(crate) fn workers_iter_mut() -> impl Iterator<Item = &'static mut Worker> {
    game().players.iter_mut().flat_map(|p| p.workers.iter_mut())
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum WalkDirection {
    Up,
    Down,
    Left,
    Right,
}

macro_rules! direction_sheets {
    ($color:expr, $base_name:ident, $action:ident) => {{
        use ak_server::types_game::Texture::*;

        let textures = match $color {
            ServerColor::Blue => &[
                concat_idents!(Blue, $base_name, $action, Up),
                concat_idents!(Blue, $base_name, $action, Down),
                concat_idents!(Blue, $base_name, $action, Left),
                concat_idents!(Blue, $base_name, $action, Right),
            ],
            ServerColor::Red => &[
                concat_idents!(Red, $base_name, $action, Up),
                concat_idents!(Red, $base_name, $action, Down),
                concat_idents!(Red, $base_name, $action, Left),
                concat_idents!(Red, $base_name, $action, Right),
            ],
            ServerColor::Green => &[
                concat_idents!(Green, $base_name, $action, Up),
                concat_idents!(Green, $base_name, $action, Down),
                concat_idents!(Green, $base_name, $action, Left),
                concat_idents!(Green, $base_name, $action, Right),
            ],
            ServerColor::Yellow => &[
                concat_idents!(Yellow, $base_name, $action, Up),
                concat_idents!(Yellow, $base_name, $action, Down),
                concat_idents!(Yellow, $base_name, $action, Left),
                concat_idents!(Yellow, $base_name, $action, Right),
            ],
        };

        hashmap! {
            WalkDirection::Up => SpriteSheet::new_12(textures[0]),
            WalkDirection::Down => SpriteSheet::new_12(textures[1]),
            WalkDirection::Left => SpriteSheet::new_12(textures[2]),
            WalkDirection::Right => SpriteSheet::new_12(textures[3]),
        }
    }};
}

/// A worker that can be controlled by the player and can build structures
#[derive(Debug, Clone, new)]
pub(crate) struct Worker {
    #[new(value = "new_id()")]
    pub(crate) id: IdType,

    #[new(value = "10")]
    pub(crate) max_hp: u16,

    #[new(value = "10")]
    pub(crate) hp: u16,

    #[new(value = "CollisionRect::new(100.0, 100.0, SQUARE_SIZE, SQUARE_SIZE)")]
    pub(crate) rect: CollisionRect,

    #[new(value = "None")]
    pub(crate) path: Option<Vec<Vec2>>,

    /// The index of the ore the worker is currently mining in the [Map]'s ore list
    #[new(value = "None")]
    pub(crate) ore: Option<usize>,

    /// Whether the worker is currently mining
    #[new(value = "false")]
    pub(crate) mining: bool,

    #[new(value = "200.0")]
    pub(crate) speed: f32,

    #[new(value = "WalkDirection::Up")]
    pub(crate) direction: WalkDirection,

    #[new(value = "0.0")]
    hspd: f32,

    #[new(value = "0.0")]
    vspd: f32,

    #[new(value = "None")]
    pub(crate) moving_away_from: Option<IdType>,

    pub(crate) color: ServerColor,

    #[new(value = "direction_sheets!(color, Worker, Walk)")]
    walk_spritesheets: FxHashMap<WalkDirection, SpriteSheet>,

    #[new(value = "direction_sheets!(color, Worker, Idle)")]
    idle_spritesheets: FxHashMap<WalkDirection, SpriteSheet>,
}

derive_id_eq!(Worker);

impl Worker {
    pub(crate) fn as_server(&self) -> ServerWorker {
        let sprite = self.spritesheet()[&self.direction].as_server();
        ServerWorker {
            pos: self.rect.top_left().into(),
            sprite,
        }
    }

    /// Sets the direction of the worker based the current `hspd` and `vspd`
    fn update_direction(&mut self) {
        let normalized = vec2(self.hspd, self.vspd);

        macro_rules! diag {
            ($first:expr, $second:expr) => {
                if self.direction == $first {
                    $first
                } else {
                    $second
                }
            };
        }

        self.direction = match (normalized.x.sign_i8(), normalized.y.sign_i8()) {
            (1, 0) => WalkDirection::Right,
            (0, -1) => WalkDirection::Up,
            (-1, 0) => WalkDirection::Left,
            (0, 1) => WalkDirection::Down,
            (1, -1) => diag!(WalkDirection::Up, WalkDirection::Right),
            (-1, -1) => diag!(WalkDirection::Up, WalkDirection::Left),
            (-1, 1) => diag!(WalkDirection::Down, WalkDirection::Left),
            (1, 1) => diag!(WalkDirection::Down, WalkDirection::Right),
            _ => self.direction,
        };
    }

    /// Returns a mutable reference to either the walk or idle spritesheets depending on the current `hspd` and `vspd`
    fn spritesheet_mut(&mut self) -> &mut FxHashMap<WalkDirection, SpriteSheet> {
        if self.hspd == 0.0 && self.vspd == 0.0 {
            &mut self.idle_spritesheets
        } else {
            &mut self.walk_spritesheets
        }
    }

    /// Returns a reference to either the walk or idle spritesheets depending on the current `hspd` and `vspd`
    fn spritesheet(&self) -> &FxHashMap<WalkDirection, SpriteSheet> {
        if self.hspd == 0.0 && self.vspd == 0.0 {
            &self.idle_spritesheets
        } else {
            &self.walk_spritesheets
        }
    }

    /// Returns a reference to the current sprite
    fn sprite(&self) -> &SpriteSheet {
        let dir = self.direction;
        self.spritesheet().get(&dir).unwrap()
    }

    /// Returns a mutable reference to the current sprite
    fn sprite_mut(&mut self) -> &mut SpriteSheet {
        let dir = self.direction;
        self.spritesheet_mut().get_mut(&dir).unwrap()
    }

    /// Moves the worker based on the current `path`. Changes `hspd` and `vspd`
    fn update_path(&mut self) {
        if let Some(path) = &mut self.path {
            if !path.is_empty() {
                let next_pos = path[0];

                let dist = distance(&self.rect.top_left(), &next_pos);
                let angle = angle(&self.rect.top_left(), &next_pos);
                let speed = self.speed * get_frame_time();

                let new_pos;
                if dist > speed {
                    new_pos = project(&self.rect.top_left(), angle, speed);
                } else {
                    new_pos = next_pos;
                    path.remove(0);
                }

                self.hspd = new_pos.x - self.rect.top_left().x;
                self.vspd = new_pos.y - self.rect.top_left().y;
                return;
            }

            self.path = None;
        }
    }

    /// Make sure the worker doesn't collide with other workers or walls, if it does, slowly move it out of the way. Changes `hspd` and `vspd`
    pub(crate) fn update_collision(&mut self) {
        for worker in
            workers_iter().filter(|w| w.id != self.id && w.moving_away_from != Some(self.id))
        {
            let mut rect = self.rect;
            rect.set_top_left(rect.top_left() + vec2(self.hspd, self.vspd));

            if rect.touches_rect(&worker.rect) {
                self.moving_away_from = Some(worker.id);

                let angle = opposite_angle(&rect.top_left(), &worker.rect.top_left());
                let speed = 50.0 * get_frame_time();
                let new_pos = project(&rect.top_left(), angle, speed);

                self.hspd += new_pos.x - rect.top_left().x;
                self.vspd += new_pos.y - rect.top_left().y;

                let mut h_rect = self.rect;
                h_rect.set_top_left(h_rect.top_left() + vec2(self.hspd, 0.0));
                let mut v_rect = self.rect;
                v_rect.set_top_left(v_rect.top_left() + vec2(0.0, self.vspd));

                for (y, row) in game().map.base_map.iter().enumerate() {
                    for (x, wall) in row.iter().enumerate() {
                        let mut collidable = false;
                        if wall == &Tile::Wall
                            || game().map.tiles.contains(&uvec2(x as u32, y as u32))
                        {
                            collidable = true;
                        }
                        if !collidable {
                            continue;
                        }

                        let wall_rect = Map::pos_to_rect(uvec2(x as u32, y as u32), 1, 1);

                        // `hspd`
                        if h_rect.touches_rect(&wall_rect) {
                            if wall_rect.center().x > h_rect.center().x {
                                self.hspd = wall_rect.left() - h_rect.right();
                            } else {
                                self.hspd = wall_rect.right() - h_rect.left();
                            }
                        }

                        // `vspd`
                        if v_rect.touches_rect(&wall_rect) {
                            if wall_rect.center().y > v_rect.center().y {
                                self.vspd = wall_rect.top() - v_rect.bottom();
                            } else {
                                self.vspd = wall_rect.bottom() - v_rect.top();
                            }
                        }
                    }
                }

                return;
            }
        }
    }

    /// Updates mining ore
    pub(crate) fn update_ore(&mut self) {
        self.mining = false;
        if let Some(i) = self.ore {
            let ore = &mut game().map.ores[i];

            let mut expanded_rect = self.rect;
            expanded_rect.expand_center(20.0, 20.0);

            if ore.as_rect().touches_rect(&expanded_rect) {
                self.path = None;
                self.mining = true;
                let amt = ore.mine(self.id);
                if amt > 0 {
                    let ore_map_ref = &mut game().player_mut(self.color).ores;
                    let ore_map = ore_map_ref.entry(ore.ore).or_insert(0);
                    *ore_map += amt;
                }
                return;
            }

            // Path to ore
            if self.path.is_none() {
                // Getting closest tile
                let top_left = ore.pos - uvec2(1, 1);
                let bottom_right = ore.pos + uvec2(ore.width, ore.height);

                // Get every tile around the ore
                let mut tiles = Vec::new();
                for y in top_left.y..=bottom_right.y {
                    for x in top_left.x..=bottom_right.x {
                        tiles.push(uvec2(x, y));
                    }
                }

                // Get the closest tile
                let closest_tile = tiles
                    .iter()
                    .min_by(|a, b| {
                        let a_dist = u_distance(&Map::world_to_pos(self.rect.top_left()), a);
                        let b_dist = u_distance(&Map::world_to_pos(self.rect.top_left()), b);
                        a_dist.partial_cmp(&b_dist).unwrap()
                    })
                    .unwrap();

                self.set_path(*closest_tile);
            }
        }
    }

    pub(crate) fn update(&mut self) {
        // Reset velocities
        self.hspd = 0.0;
        self.vspd = 0.0;

        // Movement
        self.update_ore();
        self.update_path();
        self.update_direction();
        self.update_collision();

        // Updating spritesheet
        self.sprite_mut().update();

        // Apply `hspd` and `vspd`
        self.rect
            .set_top_left(self.rect.top_left() + vec2(self.hspd, self.vspd))
    }

    /// Sets the path to the given goal position on the [crate::map::Map]
    pub(crate) fn set_path(&mut self, goal: UVec2) {
        self.path = astar(Map::world_to_pos(self.rect.top_left()), goal);
    }

    /// Draw the worker to the screen, and optionally highlight it. Also draws the path-line and time
    pub(crate) fn draw(&self, highlight: bool) {
        self.sprite().draw(self.rect.left(), self.rect.top());
        if highlight {
            self.rect.draw_lines(2.5, color_u8!(255, 255, 255, 200));
        }

        if self.mining {
            draw_texture_center_ex(
                Texture::MiningIcon.texture(),
                self.rect.center().x,
                self.rect.top() - 18.0,
                DrawTextureParams {
                    dest_size: Some(vec2(20.0, 20.0)),
                    ..Default::default()
                },
            );

            let ore = &game().map.ores[self.ore.unwrap()];
            let ratio = ore.time_left(self.id) / ore.ore.cooldown() as f32;
            let width = self.rect.width;
            let height = 2.5;
            draw_rectangle(
                self.rect.left(),
                self.rect.top() - 5.0,
                width,
                height,
                WHITE,
            );
            draw_rectangle(
                self.rect.left(),
                self.rect.top() - 5.0,
                width * (1.0 - ratio),
                height,
                RED,
            );
        }

        // Drawing path and time
        if let Some(path) = &self.path {
            if let Some(end_pos) = path.last() {
                let end_pos = *end_pos + vec2(SQUARE_SIZE / 2.0, SQUARE_SIZE / 2.0);
                draw_line(
                    self.rect.center().x,
                    self.rect.center().y,
                    end_pos.x,
                    end_pos.y,
                    2.5,
                    color_u8!(128, 128, 128, 128),
                );

                let time = path_time(&self.rect.top_left(), self.speed, path);
                let font_size = 23.0;
                draw_text_center(
                    &format!("{:.2}", time),
                    self.rect.center().x,
                    self.rect.top() - font_size / 2.0,
                    font_size,
                    WHITE,
                );
            }
        }
    }
}
