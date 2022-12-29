use std::sync::Mutex;

use derive_new::new;
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::hashmap;
use crate::types_game::{ServerMap, ServerPlayer};

lazy_static! {
    /// Map of every game to its id
    pub static ref GAMES: Mutex<FxHashMap<Uuid, Game>> = Mutex::new(hashmap! {});
    /// Map of every socket connection to their game
    pub static ref CONN_GAMES: Mutex<FxHashMap<u64, Uuid>> = Mutex::new(hashmap! {});
}

pub fn in_game(uuid: u64) -> bool {
    let games = CONN_GAMES.lock().unwrap();
    games.contains_key(&uuid)
}

#[derive(new, Clone, Serialize, Deserialize)]
pub struct Game {
    #[new(value = "Uuid::new_v4()")]
    pub uuid: Uuid,

    pub players: Vec<ServerPlayer>,

    #[new(value = "ServerMap::random()")]
    pub map: ServerMap,
}
