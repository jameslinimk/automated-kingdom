use ak_server::types_client::ClientRequest;
use ak_server::types_server::{PingResponse, ServerResponse};
use chrono::Utc;

pub fn handle_request(uuid: u64, request: ClientRequest) -> ServerResponse {
    match request {
        ClientRequest::Ping(ping) => {
            let time = ping.time;
            let now = Utc::now().timestamp_millis() as u64;

            ServerResponse::Ping(PingResponse {
                ping: now.saturating_sub(time),
            })
        }
    }
}
