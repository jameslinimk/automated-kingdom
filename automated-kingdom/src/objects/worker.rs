use std::sync::atomic::{AtomicU16, Ordering};

use ak_server::types_game::{ServerWorker, Texture};
use derive_new::new;
use macroquad::color_u8;
use macroquad::prelude::{vec2, Color, UVec2, Vec2, WHITE};
use macroquad::shapes::draw_line;
use macroquad::time::get_frame_time;
use rustc_hash::FxHashMap;

use crate::astar::{astar, path_time};
use crate::conf::SQUARE_SIZE;
use crate::game::game;
use crate::geometry::CollisionRect;
use crate::map::world_to_pos;
use crate::math::{angle, distance, project};
use crate::spritesheet::SpriteSheet;
use crate::util::{draw_text_center, FloatSignum};
use crate::{derive_id_eq, hashmap};

pub static GLOBAL_ID: AtomicU16 = AtomicU16::new(0);

/// Returns a new id for the global id system
#[inline]
pub fn new_id() -> u16 {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed)
}

/// Returns an iterator over all workers in the game
#[inline]
pub fn workers_iter() -> impl Iterator<Item = &'static Worker> {
    game().players.iter().flat_map(|p| p.workers.iter())
}

/// Returns a mutable iterator over all workers in the game
#[inline]
pub fn workers_iter_mut() -> impl Iterator<Item = &'static mut Worker> {
    game().players.iter_mut().flat_map(|p| p.workers.iter_mut())
}

#[derive(Hash, Clone, Copy, Debug, PartialEq, Eq)]
pub enum WalkDirection {
    Up,
    Down,
    Left,
    Right,
}

/// A worker that can be controlled by the player and can build structures
#[derive(Debug, Clone, new)]
pub struct Worker {
    #[new(value = "new_id()")]
    pub id: u16,

    #[new(value = "10")]
    pub max_hp: u16,

    #[new(value = "10")]
    pub hp: u16,

    #[new(value = "CollisionRect::new(100.0, 100.0, SQUARE_SIZE, SQUARE_SIZE)")]
    pub rect: CollisionRect,

    #[new(value = "None")]
    pub path: Option<Vec<Vec2>>,

    #[new(value = "200.0")]
    pub speed: f32,

    #[new(value = "WalkDirection::Up")]
    pub direction: WalkDirection,

    #[new(value = "0.0")]
    hspd: f32,

    #[new(value = "0.0")]
    vspd: f32,

    #[new(value = "hashmap! {
        WalkDirection::Up => SpriteSheet::new_12(Texture::BlueWorkerWalkUp),
        WalkDirection::Down => SpriteSheet::new_12(Texture::BlueWorkerWalkDown),
        WalkDirection::Left => SpriteSheet::new_12(Texture::BlueWorkerWalkLeft),
        WalkDirection::Right => SpriteSheet::new_12(Texture::BlueWorkerWalkRight),
    }")]
    walk_spritesheets: FxHashMap<WalkDirection, SpriteSheet>,

    #[new(value = "hashmap! {
        WalkDirection::Up => SpriteSheet::new_12(Texture::BlueWorkerIdleUp),
        WalkDirection::Down => SpriteSheet::new_12(Texture::BlueWorkerIdleDown),
        WalkDirection::Left => SpriteSheet::new_12(Texture::BlueWorkerIdleLeft),
        WalkDirection::Right => SpriteSheet::new_12(Texture::BlueWorkerIdleRight),
    }")]
    idle_spritesheets: FxHashMap<WalkDirection, SpriteSheet>,
}

derive_id_eq!(Worker);

impl Worker {
    pub fn as_server(&self) -> ServerWorker {
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

    pub fn update(&mut self) {
        // Reset velocities
        self.hspd = 0.0;
        self.vspd = 0.0;

        // Movement
        self.update_path();
        self.update_direction();

        // Updating spritesheet
        self.sprite_mut().update();

        // Apply `hspd` and `vspd`
        self.rect
            .set_top_left(self.rect.top_left() + vec2(self.hspd, self.vspd))
    }

    /// Sets the path to the given goal position on the [crate::map::Map]
    pub fn set_path(&mut self, goal: UVec2) {
        self.path = astar(world_to_pos(self.rect.top_left()), goal);
    }

    pub fn draw(&self, highlight: bool) {
        self.sprite().draw(self.rect.left(), self.rect.top());
        if highlight {
            self.rect.draw_lines(2.5, color_u8!(255, 255, 255, 200));
        }

        // Drawing time
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
