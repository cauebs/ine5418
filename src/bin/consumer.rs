use anyhow::Result;

use distribuida::{message_queue, Message, Tag};

fn main() -> Result<()> {
    let server_addrs = std::env::args()
        .skip(1)
        .next()
        .expect("Use: producer <mq_addr>:<mq_port>");

    let mq = message_queue::Client::register(server_addrs)
        .expect("Failed to register to message queue server");

    let request = Message::Request { prime_size: 42 };
    println!("sending: {:?}", &request);
    mq.send(request)?;

    let response = mq.receive::<Message>(Tag::Response)?;
    println!("received: {:?}", &response);

    Ok(())
}
