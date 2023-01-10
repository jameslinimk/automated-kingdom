use std::sync::atomic::{AtomicU16, Ordering};

use ak_server::types_game::{ServerWorker, Sprite, Texture};
use derive_new::new;
use macroquad::color_u8;
use macroquad::prelude::{vec2, Color, UVec2, Vec2, RED, WHITE};
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

    #[new(value = "None")]
    pub direction: Option<WalkDirection>,

    #[new(value = "0.0")]
    hspd: f32,

    #[new(value = "0.0")]
    vspd: f32,

    #[new(value = "hashmap! {
        WalkDirection::Up => SpriteSheet::new_fps(Texture::BlueWorkerWalkUp, 12.0),
        WalkDirection::Down => SpriteSheet::new_fps(Texture::BlueWorkerWalkDown, 12.0),
        WalkDirection::Left => SpriteSheet::new_fps(Texture::BlueWorkerWalkLeft, 12.0),
        WalkDirection::Right => SpriteSheet::new_fps(Texture::BlueWorkerWalkRight, 12.0),
    }")]
    walk_spritesheets: FxHashMap<WalkDirection, SpriteSheet>,

    #[new(value = "hashmap! {
        WalkDirection::Up => SpriteSheet::new_fps(Texture::BlueWorkerIdleUp, 12.0),
        WalkDirection::Down => SpriteSheet::new_fps(Texture::BlueWorkerIdleDown, 12.0),
        WalkDirection::Left => SpriteSheet::new_fps(Texture::BlueWorkerIdleLeft, 12.0),
        WalkDirection::Right => SpriteSheet::new_fps(Texture::BlueWorkerIdleRight, 12.0),
    }")]
    idle_spritesheets: FxHashMap<WalkDirection, SpriteSheet>,
}

derive_id_eq!(Worker);

impl Worker {
    pub fn as_server(&self) -> ServerWorker {
        todo!();
        // ServerWorker {
        //     pos: self.rect.top_left().into(),
        //     sprite: self.sprite.as_server(),
        // }
    }

    /// Sets the direction of the worker based on an angle in radians
    fn update_direction(&mut self) {
        let normalized = vec2(self.hspd, self.vspd);

        macro_rules! diag {
            ($first:expr, $second:expr) => {
                if self.direction == Some($first) {
                    Some($first)
                } else {
                    Some($second)
                }
            };
        }

        self.direction = match (normalized.x.sign_i8(), normalized.y.sign_i8()) {
            (1, 0) => Some(WalkDirection::Right),
            (0, -1) => Some(WalkDirection::Up),
            (-1, 0) => Some(WalkDirection::Left),
            (0, 1) => Some(WalkDirection::Down),
            (1, -1) => diag!(WalkDirection::Up, WalkDirection::Right),
            (-1, -1) => diag!(WalkDirection::Up, WalkDirection::Left),
            (-1, 1) => diag!(WalkDirection::Down, WalkDirection::Left),
            (1, 1) => diag!(WalkDirection::Down, WalkDirection::Right),
            _ => self.direction,
        };
    }

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
        self.hspd = 0.0;
        self.vspd = 0.0;

        self.update_path();
        self.update_direction();
        println!("self.direction: {:?}", self.direction);

        // Applying `hspd` and `vspd`
        self.rect
            .set_top_left(self.rect.top_left() + vec2(self.hspd, self.vspd))
    }

    /// Sets the path to the given goal position on the map
    pub fn path_to(&mut self, goal: UVec2) {
        let path = astar(world_to_pos(self.rect.top_left()), goal);
        self.path = path;
    }

    pub fn draw(&self, highlight: bool) {
        self.rect.draw(RED);
        if highlight {
            self.rect.draw_lines(5.0, color_u8!(255, 255, 255, 200));
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
