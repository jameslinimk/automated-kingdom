//! Functions for astar pathfinding

use std::hash::BuildHasherDefault;

use macroquad::prelude::{uvec2, UVec2, Vec2};
use priority_queue::PriorityQueue;
use rustc_hash::FxHasher;

use crate::hashmap;
use crate::map::{loc_to_world, Map, Tile};
use crate::math::distance;

/// Gets the manhattan distance between two points
fn manhattan_distance(from: &UVec2, to: &UVec2) -> u32 {
    let x_dis = from.x.abs_diff(to.x);
    let y_dis = from.y.abs_diff(to.y);
    x_dis + y_dis
}

/// Checks if a point is valid to move to
fn valid(point: &UVec2, map: &Map) -> bool {
    point.x < map.width as u32 && point.y < map.height as u32 && map.get(point) == Tile::Air
}

/// Returns a list of valid moves from a point
fn neighbors(point: &UVec2, map: &Map) -> Vec<UVec2> {
    let mut children = vec![];

    macro_rules! add_if_valid {
        ($x_diff: expr, $y_diff: expr) => {
            if point.x as i32 + $x_diff >= 0 && point.y as i32 + $y_diff >= 0 {
                let new_point = uvec2(
                    (point.x as i32 + $x_diff) as u32,
                    (point.y as i32 + $y_diff) as u32,
                );
                if valid(&new_point, map) {
                    children.push(new_point);
                }
            }
        };
    }

    add_if_valid!(0, 1);
    add_if_valid!(0, -1);
    add_if_valid!(1, 0);
    add_if_valid!(-1, 0);

    macro_rules! add_if_valid_diagonals {
        ($x_diff: expr, $y_diff: expr) => {
            if point.x as i32 + $x_diff >= 0
                && point.y as i32 + $y_diff >= 0
                && point.x as i32 + $x_diff < map.width as i32
                && point.y as i32 + $y_diff < map.height as i32
            {
                if map.get(&uvec2((point.x as i32 + $x_diff) as u32, point.y)) == Tile::Air
                    && map.get(&uvec2(point.x, (point.y as i32 + $y_diff) as u32)) == Tile::Air
                {
                    add_if_valid!($x_diff, $y_diff);
                }
            }
        };
    }

    add_if_valid_diagonals!(1, 1);
    add_if_valid_diagonals!(1, -1);
    add_if_valid_diagonals!(-1, 1);
    add_if_valid_diagonals!(-1, -1);

    children
}

/// Returns a path from start to goal using the A* algorithm, or `None` if no path is found
pub fn astar(start: &UVec2, goal: &UVec2, map: &Map) -> Option<Vec<Vec2>> {
    println!("(start, goal): {:?}", (start, goal));

    let mut parents = hashmap! {};
    let mut costs = hashmap! {};
    let mut priority_queue = PriorityQueue::<UVec2, u32, BuildHasherDefault<FxHasher>>::default();
    let mut current = *start;

    priority_queue.push(current, 0);
    parents.insert(current, current);
    costs.insert(current, 0);

    while let Some((_current, _)) = priority_queue.pop() {
        current = _current;
        if current == *goal {
            break;
        }

        for neighbor in neighbors(&current, map).iter() {
            let new_cost = costs[&current] + 1;
            if !costs.contains_key(neighbor) || new_cost < costs[neighbor] {
                costs.insert(*neighbor, new_cost);
                let priority = u32::MAX - (new_cost + manhattan_distance(neighbor, goal));

                priority_queue.push(*neighbor, priority);
                parents.insert(*neighbor, current);
            }
        }
    }

    let mut path = vec![];
    if costs.contains_key(goal) {
        while current != *start {
            path.push(loc_to_world(&current));
            current = parents[&current];
        }
        path.push(loc_to_world(start));
        path.reverse();
        return Some(path);
    }

    None
}

/// Calculates the time it takes to travel along a path
pub fn path_time(current_pos: &Vec2, speed: f32, path: &[Vec2]) -> f32 {
    let mut time = 0.0;
    if !path.is_empty() {
        time += distance(current_pos, &path[0]);
        for i in 1..path.len() - 1 {
            let dist = distance(&path[i], &path[i + 1]);
            time += dist;
        }
        time /= speed;
    }
    time
}

#[test]
fn test_astar() {
    // Write tests for astar function
    let width = 10;
    let height = 10;
    let mut test_map = Map {
        width,
        height,
        map: vec![vec![Tile::Air; width]; height],
    };

    test_map.map[8][9] = Tile::Wall;
    // test_map.map[9][8] = Tile::Wall;
    test_map.map[8][8] = Tile::Wall;

    // Print map
    for row in test_map.map.iter() {
        for tile in row.iter() {
            match tile {
                Tile::Wall => print!("#"),
                Tile::Air => print!("."),
            }
        }
        println!();
    }

    let goal = uvec2(9, 9);
    let start = uvec2(0, 0);
    let path = astar(&start, &goal, &test_map).unwrap();
    println!("path: {:?}", path);
}
