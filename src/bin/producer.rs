use anyhow::Result;
use distribuida::{message_queue, Message, Tag};

fn main() -> Result<()> {
    let mq = message_queue::Client::new("127.0.0.1:8979");

    let request = mq.receive::<Message>(Tag::Request)?;
    println!("received: {:?}", &request);

    let response = Message::Response {
        recipient: request.sender,
        prime: vec![0, 1, 2, 3],
    };
    println!("sending: {:?}", &response);
    mq.send(response)?;

    Ok(())
}
