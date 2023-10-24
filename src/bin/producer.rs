use std::time::SystemTime;

use distribuida::{message_queue, Message, Tag};
use log::{error, info, warn};

use ine5429_primes::functions;

fn main() {
    env_logger::init();
    info!("Producer initializing... ");
    let mq = message_queue::Client::new("127.0.0.1:8979");
    loop {
        let request = mq.receive::<Message>(Tag::Request).unwrap();
        let seed = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        info!("received: {:?}", &request);
        info!("Producer initialized with seed: {}", &seed);

        let prime_size = match request.inner {
            Message::Request { prime_size: v } => v,
            Message::Response { .. } => {
                warn!("Unexpected response");
                continue;
            }
        };

        let prime = functions::find_fermat(prime_size, &seed.into());

        let response = Message::Response {
            recipient: request.sender,
            prime: prime.to_bytes_le(),
        };
        info!("sending: {:?}", &response);
        match mq.send(response) {
            Ok(_) => info!("Response successfully sent"),
            Err(e) => error!("Failed to respond request: {:?}", e),
        }
    }
}
