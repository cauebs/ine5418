use distribuida::{message_queue, Message, Tag};
use ine5429_primes::functions;

use std::time::SystemTime;

fn generate_prime(prime_size: u32) -> Vec<u8> {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros();

    functions::find_fermat(u64::from(prime_size), &seed.into()).to_bytes_le()
}

fn main() {
    env_logger::init();
    log::info!("Producer initializing... ");

    let server_addrs = std::env::args()
        .skip(1)
        .next()
        .expect("Use: producer <mq_addr>:<mq_port>");

    let mq = message_queue::Client::register(server_addrs)
        .expect("Failed to register to message queue server");

    loop {
        let request = mq.receive::<Message>(Tag::Request).unwrap();
        log::info!("Received message: {:?}", &request);

        let prime_size = match request.inner {
            Message::Request { prime_size: v } => v,
            Message::Response { .. } => {
                log::error!("Asked for a Request, but got a Response");
                continue;
            }
        };

        let prime = generate_prime(prime_size);

        let response = Message::Response {
            recipient: request.sender,
            prime,
        };
        log::info!("Sending message: {:?}", &response);
        match mq.send(response) {
            Ok(_) => log::info!("Response successfully sent"),
            Err(e) => log::error!("Failed to respond request: {e:?}"),
        }
    }
}
