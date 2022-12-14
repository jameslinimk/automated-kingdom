use std::sync::atomic::{AtomicU16, Ordering};
use std::sync::{Mutex, MutexGuard};

use derive_new::new;
use lazy_static::lazy_static;
use macroquad::color_u8;
use macroquad::prelude::{Color, Vec2, RED, WHITE};
use macroquad::shapes::draw_line;
use macroquad::time::get_frame_time;
use rustc_hash::FxHashMap;

use crate::astar::path_time;
use crate::conf::{get_font_small, SQUARE_SIZE};
use crate::geometry::CollisionRect;
use crate::hashmap;
use crate::math::{angle, distance, project};
use crate::util::draw_centered_text;

pub static GLOBAL_ID: AtomicU16 = AtomicU16::new(0);
pub fn get_new_id() -> u16 {
    GLOBAL_ID.fetch_add(1, Ordering::Relaxed)
}

lazy_static! {
    /// A map of all workers in the game indexed by their id
    static ref WORKERS: Mutex<FxHashMap<u16, Worker>> = Mutex::new(hashmap! {});
}

/// Creates a mutable reference to the [WORKERS] map
#[inline]
pub fn get_workers() -> MutexGuard<'static, FxHashMap<u16, Worker>> {
    WORKERS.lock().unwrap()
}

/// Creates a mutable reference to a new worker
#[macro_export]
macro_rules! new_worker {
    ($name: ident) => {{
        let index = $crate::objects::worker::Worker::new_add();
        get_worker!($name, index);
    }};
}

/// Creates a mutable reference to a worker with the given id
#[macro_export]
macro_rules! get_worker {
    ($name: ident, $index: expr) => {
        let mut binding = $crate::objects::worker::get_workers();
        let $name = binding.get_mut(&$index).unwrap();
    };
}

#[derive(Debug, Clone, new)]
/// A worker that can be controlled by the player and can build structures
pub struct Worker {
    #[new(value = "get_new_id()")]
    pub id: u16,

    #[new(value = "10")]
    pub max_hp: u16,

    #[new(value = "10")]
    pub hp: u16,

    #[new(value = "CollisionRect::new(100.0, 100.0, SQUARE_SIZE, SQUARE_SIZE)")]
    pub rect: CollisionRect,

    #[new(value = "None")]
    pub path: Option<Vec<Vec2>>,

    #[new(value = "500.0")]
    pub speed: f32,

    #[new(value = "true")]
    pub draw_path: bool,
}
impl Worker {
    /// Creates a new worker and adds it to the global [WORKERS] map. Returns the worker's id
    pub fn new_add() -> u16 {
        let worker = Worker::new();
        let id = worker.id;
        WORKERS.lock().unwrap().insert(worker.id, worker);
        id
    }

    pub fn update(&mut self) {
        /* --------------------------------- Pathing -------------------------------- */
        if let Some(path) = &mut self.path {
            if !path.is_empty() {
                let next_pos = path[0];

                let dist = distance(&self.rect.center(), &next_pos);
                let angle = angle(&self.rect.center(), &next_pos);
                let speed = self.speed * get_frame_time();

                if dist > speed {
                    self.rect
                        .set_center(project(&self.rect.center(), angle, speed));
                } else {
                    self.rect.set_center(next_pos);
                    path.remove(0);
                }

                // Drawing time
                if self.draw_path {
                    draw_line(
                        self.rect.center().x,
                        self.rect.center().y,
                        next_pos.x,
                        next_pos.y,
                        2.5,
                        color_u8!(128, 128, 128, 128),
                    );

                    let time = path_time(&self.rect.center(), self.speed, path);
                    draw_centered_text(
                        &format!("{:.2}", time),
                        self.rect.center().x,
                        self.rect.top(),
                        get_font_small(),
                        17.0,
                        WHITE,
                    );
                }
            } else {
                self.path = None;
            }
        }
    }

    pub fn draw(&self, highlight: bool) {
        self.rect.draw(RED);
        if highlight {
            self.rect.draw_lines(2.0, color_u8!(255, 255, 255, 128));
        }
    }
}
