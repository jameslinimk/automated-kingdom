use std::net::{SocketAddr, UdpSocket};
use std::thread::sleep;
use std::time::Duration;

use ak_server::types_client::{ClientRequest, Ping};
use ak_server::types_server::ServerResponse;
use chrono::Utc;

pub fn main() {
    let remote_addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let socket = UdpSocket::bind("127.0.0.1:8081").expect("Could not bind socket");

    loop {
        // Send a ping
        let test_payload = ClientRequest::Ping(Ping {
            timestamp: Utc::now().timestamp_millis() as u64,
        });
        let test_payload = rmp_serde::to_vec(&test_payload).unwrap();
        socket.send_to(&test_payload, remote_addr).unwrap();

        let mut buf = [0; 1024];
        let (n, _) = socket.recv_from(&mut buf).expect("Didn't receive data");
        let filled_buf = &mut buf[..n];

        println!("line: {:?}", filled_buf);

        let (data, ping) = rmp_serde::from_slice::<ServerResponse>(filled_buf).unwrap();
        println!("{:?} (ping: {})", data, ping);

        sleep(Duration::from_millis(1500));
    }
}
