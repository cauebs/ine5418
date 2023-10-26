use anyhow::Result;
use distribuida::{message_queue, Message, Tag};

fn main() -> Result<()> {
    let mq = message_queue::Client::new(
        std::env::args()
            .skip(1)
            .next()
            .expect("Use: consumer <host>:<port>"),
    );

    let request = Message::Request { prime_size: 42 };
    println!("sending: {:?}", &request);
    mq.send(request)?;

    let response = mq.receive::<Message>(Tag::Response)?;
    println!("received: {:?}", &response);

    Ok(())
}
