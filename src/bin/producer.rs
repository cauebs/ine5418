use ine5429_primes::functions;

use distribuida::{message_queue, PrimesMessage, PrimesTag};
use num_bigint::BigUint;
use num_bigint::ToBigUint;

use std::{net::ToSocketAddrs, thread, time::SystemTime};

fn generate_prime(prime_size: u32) -> BigUint {
    let seed = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_micros();

    if prime_size < 16 {
        ToBigUint::to_biguint(&0).unwrap()
    } else {
        functions::find_fermat(u64::from(prime_size), &seed.into())
    }
}

fn run_producer<A: ToSocketAddrs>(server_addrs: A) {
    let tid = thread::current().id();

    let mq = message_queue::Client::register(server_addrs)
        .map_err(|e| log::error!("[{tid:?}] Failed to register to message queue server: {e:?}"))
        .unwrap();

    loop {
        let request = mq
            .receive::<PrimesMessage>(PrimesTag::Request)
            .map_err(|e| log::error!("[{tid:?}] Failed to receive request: {e:?}"))
            .unwrap();
        log::info!("[{tid:?}] Received message: {:?}", &request);

        let PrimesMessage::Request { prime_size } = request.inner else {
            log::error!("[{tid:?}] Asked for a Request, but got a Response!");
            continue;
        };

        let prime = generate_prime(prime_size);
        let prime: Vec<u8> = prime.to_bytes_le();

        let response = PrimesMessage::Response {
            recipient: request.sender,
            prime,
        };
        log::info!("[{tid:?}] Sending message: {:?}", &response);
        match mq.send(response) {
            Ok(_) => log::info!("[{tid:?}] Response successfully sent"),
            Err(e) => log::error!("[{tid:?}] Failed to respond request: {e:?}"),
        }
    }
}

fn main() {
    env_logger::init();
    log::info!("Producer initializing...");

    let mut args = std::env::args().skip(1);

    let server_addrs = args
        .next()
        .expect("Use: producer <mq_addr>:<mq_port> [num_threads]");

    let num_threads = args.next().and_then(|s| s.parse::<u32>().ok()).unwrap_or(1);
    log::info!("Starting {num_threads} thread(s)");
    thread::scope(|s| {
        for _ in 0..num_threads {
            s.spawn(|| run_producer(&server_addrs));
        }
    });
}
