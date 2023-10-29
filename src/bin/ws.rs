use axum::{
    routing::get,
    extract::Path, Router,
};
use distribuida::{message_queue, PrimesMessage, PrimesTag};
use log::{info, warn};
use num_bigint::BigUint;

#[tokio::main]
pub async fn main() {
    env_logger::init();

    let mut args = std::env::args().skip(1);
    let addrs = args
        .next()
        .expect("Use: ws <bind_addr>:<port> <mq_addr>:<port>");

    let app = Router::new()
        .route("/:psize", get(root));

    let listener = tokio::net::TcpListener::bind(addrs)
        .await
        .unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root(Path(psize): Path<u32>) ->  String {
    info!("Handling request for {} sized prime", psize);

    let mut args = std::env::args().skip(2);

    let addrs = args
        .next()
        .expect("Use: ws <wq_addr>:<port>");
    
    let mq = message_queue::Client::register(addrs).unwrap();

    let request = PrimesMessage::Request { prime_size: psize };
    log::debug!("sending: {:?}", &request);
    match mq.send_with_retry(request) {
        Ok(_) => {}
        Err(e) => {
            warn!("Error while sending request: {e}");
            return "Failed to send request. Try again soon.".to_string()
        },
    }

    let Ok(response) = mq.receive::<PrimesMessage>(PrimesTag::Response) else {
        warn!("Error while receiving response. Try again.");
        return "Request failed. Try again soon.".to_string()
    };

    log::debug!("received: {:?}", &response);

    let PrimesMessage::Response { prime, .. } = response.inner else {
        warn!("Asked for a Response, but got a Request!");
        return "Unexpected error".to_string();
    };

    BigUint::from_bytes_le(&prime).to_string()
}