use std::sync::atomic::{AtomicU16, Ordering};

use derive_new::new;
use macroquad::color_u8;
use macroquad::prelude::{vec2, Color, UVec2, Vec2, RED, WHITE};
use macroquad::shapes::draw_line;
use macroquad::time::get_frame_time;

use crate::astar::{astar, path_time};
use crate::conf::SQUARE_SIZE;
use crate::game::game;
use crate::geometry::CollisionRect;
use crate::map::world_to_pos;
use crate::math::{angle, distance, project};
use crate::util::draw_text_center;

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

#[derive(Clone, Copy, Debug)]
pub enum Direction {
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
    pub direction: Option<Direction>,
}
impl PartialEq for Worker {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Worker {
    pub fn update(&mut self) {
        /* --------------------------------- Pathing -------------------------------- */
        if let Some(path) = &mut self.path {
            if !path.is_empty() {
                let next_pos = path[0];

                let dist = distance(&self.rect.top_left(), &next_pos);
                let angle = angle(&self.rect.top_left(), &next_pos);
                let speed = self.speed * get_frame_time();

                let dir = self.rect.top_left().signum() - next_pos.signum();
                self.direction = Some(match (dir.x as i8, dir.y as i8) {
                    (0, 1) => Direction::Up,
                    (0, -1) => Direction::Down,
                    (1, 0) => Direction::Left,
                    (-1, 0) => Direction::Right,
                    _ => unreachable!(),
                });

                println!("self.direction: {:?}", self.direction);

                if dist > speed {
                    self.rect
                        .set_top_left(project(&self.rect.top_left(), angle, speed));
                } else {
                    self.rect.set_top_left(next_pos);
                    path.remove(0);
                }
            } else {
                self.path = None;
            }
        }
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

                let time = path_time(&self.rect.center(), self.speed, path);
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
