use std::sync::Mutex;

use ak_server::game::{in_game, Game, CONN_GAMES, GAMES};
use ak_server::hashmap;
use ak_server::types_client::ClientRequest;
use ak_server::types_game::{Color, Player};
use ak_server::types_server::{ErrorCode, ServerResponseData};
use lazy_static::lazy_static;
use rustc_hash::FxHashMap;

use crate::add_username;

lazy_static! {
    static ref CONN_RATELIMIT: Mutex<FxHashMap<u64, u64>> = Mutex::from(hashmap! {});
}

/// The minimum amount of time between requests that are ratelimited
const RATELIMIT: u64 = 250;

fn valid_password(input: &str) -> Option<ErrorCode> {
    if input.len() <= 50 {
        return Some(ErrorCode::UsernameTooLong);
    }

    let valid_chars = " abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890!@#$%^&*()_+-=[]{};':\",./<>?\\|`~";
    if input.chars().all(|c| valid_chars.contains(c)) {
        return Some(ErrorCode::UsernameInvalid);
    }

    None
}

pub fn handle_request(uuid: u64, request: &ClientRequest) -> ServerResponseData {
    // Check if request is ratelimited
    if request.ratelimited() {
        let mut ratelimits = CONN_RATELIMIT.lock().unwrap();
        if let Some(last_req) = ratelimits.get(&uuid) {
            if *last_req + RATELIMIT > request.timestamp() {
                return ServerResponseData::Error(ErrorCode::Ratelimited);
            }
        }
        ratelimits.insert(uuid, request.timestamp());
    }

    match request {
        ClientRequest::Ping(_) => ServerResponseData::Success,
        ClientRequest::Rename(rename) => {
            // Make sure name is valid
            if let Some(err) = valid_password(&rename.name) {
                return ServerResponseData::Error(err);
            }

            add_username(uuid, &rename.name);
            ServerResponseData::Success
        }
        ClientRequest::CreateGame(_) => {
            if in_game(uuid) {
                return ServerResponseData::Error(ErrorCode::AlreadyInGame);
            }

            let mut game = Game::default();
            game.players.push(Player {
                uuid,
                ping: 0,
                workers: vec![],
                color: Color::Blue,
            });

            let mut conn_games = CONN_GAMES.lock().unwrap();
            conn_games.insert(uuid, game.uuid);

            let mut games = GAMES.lock().unwrap();
            games.insert(game.uuid, game);

            todo!()
        }
    }
}
