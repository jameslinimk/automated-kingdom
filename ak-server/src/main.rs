use std::fs;
use std::hash::{Hash, Hasher};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

use ak_server::hashmap;
use ak_server::types_client::ClientRequest;
use colored::Colorize;
use lazy_static::lazy_static;
use rustc_hash::{FxHashMap, FxHasher};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

use crate::handle_request::handle_request;

mod handle_request;

lazy_static! {
    static ref CONN_COUNT: AtomicU64 = AtomicU64::new(0);
    static ref CONN_USERNAMES: Mutex<FxHashMap<u64, String>> = Mutex::from(hashmap! {});
}

fn add_username(uuid: u64, username: &str) {
    let mut usernames = CONN_USERNAMES.lock().unwrap();
    usernames.insert(uuid, username.to_string());

    let json = serde_json::to_string(&*usernames).unwrap();
    fs::write("usernames.json", json).unwrap();
}

/// Close a connection, remove it from the connection count, and return
macro_rules! close_return {
    () => {{
        CONN_COUNT.fetch_sub(1, Ordering::Relaxed);
        return;
    }};
    ($($arg:tt)*) => {{
        println!("{}", format!($($arg)*).red());
        CONN_COUNT.fetch_sub(1, Ordering::Relaxed);
        return;
    }};
}

fn hash_socket(socket: &TcpStream) -> Result<u64, std::io::Error> {
    let mut hash = FxHasher::default();
    let source_addr = socket.local_addr()?;
    let dest_addr = socket.peer_addr()?;

    source_addr.hash(&mut hash);
    dest_addr.hash(&mut hash);

    Ok(hash.finish())
}

#[allow(dead_code)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load usernames
    match fs::read("usernames.json") {
        Ok(json) => {
            let usernames: FxHashMap<u64, String> = serde_json::from_slice(&json)?;
            *CONN_USERNAMES.lock().unwrap() = usernames;
            println!("{}", "Successfully loaded usernames.json".green());
        }
        Err(_) => {
            println!(
                "{}",
                "No usernames.json not found, creating new file...".yellow()
            );
            fs::write("usernames.json", "{}")?;
        }
    };

    // Start listening
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!(
        "{}",
        format!("Listening on: {}", listener.local_addr()?).green()
    );

    // Accept connections and process them, spawning a new tasks for each one
    loop {
        let (mut socket, _) = listener.accept().await?;
        let hash = hash_socket(&socket)?;

        println!(
            "{}",
            format!(
                "New connection: {}, total connections: {}",
                hash,
                CONN_COUNT.load(Ordering::Relaxed) + 1
            )
            .green()
        );

        CONN_COUNT.fetch_add(1, Ordering::Relaxed);

        // Add username
        let username = format!("Guest-{}", 5);
        add_username(hash, &username);

        tokio::spawn(async move {
            let mut buf = [0; 1024];

            loop {
                let n = match socket.read(&mut buf).await {
                    // socket closed
                    Ok(n) if n == 0 => close_return!(),
                    Ok(n) => n,
                    Err(err) => {
                        close_return!("Failed to read from socket; {:?}", err);
                    }
                };

                let raw = &buf[0..n];
                println!("Received: {:?}", raw);

                let request: ClientRequest = match rmp_serde::from_slice(raw) {
                    Ok(response) => response,
                    Err(err) => {
                        close_return!("Failed to deserialize response; {:?}", err);
                    }
                };

                let response = handle_request(hash, request);
                let mut response = rmp_serde::to_vec(&response).unwrap();

                // Prepend the length of the response to the response
                response.splice(0..0, response.len().to_ne_bytes().iter().copied());

                println!("Sending: {:?}", response);
                if let Err(err) = socket.write_all(&response).await {
                    close_return!("Failed to write to socket; {:?}", err);
                }
            }
        });
    }
}
