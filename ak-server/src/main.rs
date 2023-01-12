use std::hash::{Hash, Hasher};
use std::net::SocketAddr;
use std::sync::Mutex;

use ak_server::game::{in_game, CONN_GAMES};
use ak_server::types_client::ClientRequest;
use ak_server::{hashmap, hashset};
use chrono::Utc;
use colored::Colorize;
use lazy_static::lazy_static;
use rustc_hash::{FxHashMap, FxHashSet, FxHasher};
use tokio::net::UdpSocket;

use crate::handle_request::handle_request;

mod handle_request;

lazy_static! {
    static ref CONN_COUNT: Mutex<FxHashSet<u64>> = Mutex::from(hashset! {});
    static ref CONN_USERNAMES: Mutex<FxHashMap<u64, String>> = Mutex::from(hashmap! {});
}

fn add_username(uuid: u64, username: &str) {
    let mut usernames = CONN_USERNAMES.lock().unwrap();
    usernames.insert(uuid, username.to_string());
}

fn hash_addr(addr: SocketAddr) -> u64 {
    let host = addr.ip();
    let port = addr.port();

    let mut hasher = FxHasher::default();
    (host, port).hash(&mut hasher);

    hasher.finish()
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Start listening
    let socket = UdpSocket::bind("127.0.0.1:8080").await?;
    println!(
        "{}",
        format!("Listening on: {}", socket.local_addr()?).green()
    );

    let mut buf = [0; 1024];

    // Accept connections and process them, spawning a new tasks for each one
    loop {
        let (n, addr) = socket.recv_from(&mut buf).await?;
        let hash = hash_addr(addr);

        CONN_COUNT.lock().unwrap().insert(hash);
        println!(
            "{}",
            format!(
                "New connection: {}, total connections: {}",
                hash,
                CONN_COUNT.lock().unwrap().len()
            )
            .green()
        );

        // Add username
        let username = format!("Guest-{}", (hash & 0xFFFF));
        add_username(hash, &username);

        /// Close a connection, remove it from the connection count, remove player from game if in one, and return
        macro_rules! close_return {
            () => {{
                CONN_COUNT.lock().unwrap().remove(&hash);
                if in_game(hash) {
                    CONN_GAMES.lock().unwrap().remove(&hash);
                }
                continue;
            }};
            ($($arg:tt)*) => {{
                println!("{}", format!($($arg)*).red());
                close_return!();
            }};
        }

        let raw = &buf[0..n];
        let request: ClientRequest = match rmp_serde::from_slice(raw) {
            Ok(response) => response,
            Err(err) => {
                close_return!("Failed to deserialize response; {:?}", err);
            }
        };

        let now = Utc::now().timestamp_millis() as u64;
        let ping = now.saturating_sub(request.timestamp());
        let response = (
            handle_request(hash, &request),
            ping.clamp(0, u16::MAX.into()) as u16,
        );
        let response = rmp_serde::to_vec(&response).unwrap();

        if let Err(err) = socket.send_to(&response, addr).await {
            close_return!("Failed to write to socket; {:?}", err);
        }
    }
}
