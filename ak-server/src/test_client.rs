use std::io::{BufReader, Read, Write};
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;

use ak_server::types_client::{ClientRequest, Ping};
use ak_server::types_server::ServerResponse;
use chrono::Utc;

pub fn main() {
    let mut stream = TcpStream::connect("127.0.0.1:8080").unwrap();

    loop {
        // Send a ping
        let test_payload = ClientRequest::Ping(Ping {
            time: Utc::now().timestamp_millis() as u64,
        });
        let test_payload = rmp_serde::to_vec(&test_payload).unwrap();
        stream.write_all(&test_payload).unwrap();

        let mut reader = BufReader::new(&stream);

        // Get first 8 bytes of the reader (length of the rest of the reader)
        let mut buffer = [0; 8];
        reader.read_exact(&mut buffer).unwrap();
        let res_len = usize::from_ne_bytes(buffer);

        // Get the rest of the reader
        let mut res = vec![0; res_len];
        reader.read_exact(&mut res).unwrap();

        println!("line: {:?}", res);

        let data: ServerResponse = rmp_serde::from_slice(&res).unwrap();
        println!("{:?}", data);

        sleep(Duration::from_millis(1500));
    }
}
