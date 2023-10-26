use std::time::SystemTime;

use distribuida::{message_queue, Message, Tag};
use log::{error, info, warn};

use ine5429_primes::functions;

fn generate_prime(prime_size: u32) -> Vec<u8> {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros();

    functions::find_fermat(u64::from(prime_size), &seed.into()).to_bytes_le()
}

fn main() {
    env_logger::init();
    info!("Producer initializing... ");

    let mq = message_queue::Client::new(
        std::env::args()
            .skip(1)
            .next()
            .expect("Use: producer <host>:<port>"),
    );

    loop {
        let request = mq.receive::<Message>(Tag::Request).unwrap();
        info!("received: {:?}", &request);

        let prime_size = match request.inner {
            Message::Request { prime_size: v } => v,
            Message::Response { .. } => {
                warn!("Unexpected response");
                continue;
            }
        };

        let prime = generate_prime(prime_size);

        let response = Message::Response {
            recipient: request.sender,
            prime,
        };
        info!("sending: {:?}", &response);
        match mq.send(response) {
            Ok(_) => info!("Response successfully sent"),
            Err(e) => error!("Failed to respond request: {:?}", e),
        }
    }
}
