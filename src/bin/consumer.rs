use std::io::stdin;

use anyhow::Result;

use log::{info,warn};

use distribuida::{message_queue, Message, Tag};

fn ask_prime(mq: &message_queue::Client) -> Result<()> {
    let request = Message::Request { prime_size: 42 };
    info!("sending: {:?}", &request);
    mq.send(request)?;
    Ok(())
}

fn get_prime(mq: &message_queue::Client) -> Result<()> {
    let response = mq.receive::<Message>(Tag::Response)?;
    info!("received: {:?}", &response);
    Ok(())
}

fn main() -> Result<()> {
    let server_addrs = std::env::args()
        .skip(1)
        .next()
        .expect("Use: consumer <mq_addr>:<mq_port>");

    env_logger::init();

    let mq = message_queue::Client::register(server_addrs)
        .expect("Failed to register to message queue server");

    info!("***Prime numbers client***");
    loop {
        let mut input = String::new();
        let _ = stdin().read_line(&mut input);

        let _ = match input.trim() {
            "ask" => ask_prime(&mq),
            "get" => get_prime(&mq),
            "exit" => break,
            m => {
                warn!("{} is not ah valid command", m);
                Ok(())
            },
        };
    }

    info!("***Client finished***");
    Ok(())
}
