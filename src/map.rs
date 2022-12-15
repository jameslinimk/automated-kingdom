use derive_new::new;
use macroquad::prelude::{uvec2, vec2, UVec2, Vec2, WHITE};
use macroquad::shapes::draw_rectangle;

use crate::conf::SQUARE_SIZE;

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Tile {
    Wall,
    Air,
}

#[derive(Clone, PartialEq, Debug, new)]
pub struct Map {
    #[new(value = "string_to_map(TEST_MAP).0")]
    pub map: Vec<Vec<Tile>>,

    #[new(value = "40")]
    pub width: usize,

    #[new(value = "12")]
    pub height: usize,
}
impl Map {
    pub fn get(&self, loc: &UVec2) -> Tile {
        self.map[loc.y as usize][loc.x as usize]
    }

    pub fn draw(&self) {
        for (y, row) in self.map.iter().enumerate() {
            for (x, tile) in row.iter().enumerate() {
                let world_loc = loc_to_world(&uvec2(x as u32, y as u32));
                match tile {
                    Tile::Wall => {
                        draw_rectangle(world_loc.x, world_loc.y, SQUARE_SIZE, SQUARE_SIZE, WHITE);
                    }
                    Tile::Air => {}
                }
            }
        }
    }
}

/// Inverse of [world_to_loc]. Converts a location on a [Map] to a world position
pub fn loc_to_world(loc: &UVec2) -> Vec2 {
    vec2(loc.x as f32 * SQUARE_SIZE, loc.y as f32 * SQUARE_SIZE)
}

/// Inverse of [loc_to_world], converts a world position to a location on a [Map]
pub fn world_to_loc(loc: &Vec2) -> UVec2 {
    uvec2((loc.x / SQUARE_SIZE) as u32, (loc.y / SQUARE_SIZE) as u32)
}

fn string_to_map(string: &'static str) -> (Vec<Vec<Tile>>, usize, usize) {
    let mut map = vec![];
    for line in string.lines() {
        let mut row = vec![];
        for c in line.chars() {
            match c {
                '#' => row.push(Tile::Wall),
                '.' => row.push(Tile::Air),
                _ => panic!("Invalid character in map string"),
            }
        }
        map.push(row);
    }
    let width = map[0].len();
    let height = map.len();
    (map, width, height)
}

#[test]
fn test() {
    let (_, width, height) = string_to_map(TEST_MAP);
    println!("(width, height): {:?}", (width, height));
}

const TEST_MAP: &str = "........................................
..............................#.........
..............................#.........
..............................#.........
..............................#.........
..............................#.........
..............................#.........
..............................#.........
..............................#.........
..............................#.........
..............................#.........
..............................#.........";
