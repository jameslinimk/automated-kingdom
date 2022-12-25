use std::sync::Mutex;

use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::hashmap;
use crate::types_game::{Map, Player};

lazy_static! {
    pub static ref GAMES: Mutex<FxHashMap<Uuid, Game>> = Mutex::new(hashmap! {});
    pub static ref CONN_GAMES: Mutex<FxHashMap<u64, Uuid>> = Mutex::new(hashmap! {});
}

pub fn in_game(uuid: u64) -> bool {
    let games = CONN_GAMES.lock().unwrap();
    games.contains_key(&uuid)
}

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub uuid: Uuid,
    pub players: Vec<Player>,
    pub map: Map,
}
impl Default for Game {
    fn default() -> Game {
        Game {
            uuid: Uuid::new_v4(),
            players: vec![],
            map: Map {
                tiles: vec![],
                width: 0,
                height: 0,
            },
        }
    }
}
impl Game {}
