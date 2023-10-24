use std::{
    io::stdin,
    net::ToSocketAddrs,
};

use anyhow::Result;
use distribuida::{message_queue, Message, Tag};

fn ask_prime(mq: message_queue::Client) -> Result<()> {
    let request = Message::Request { prime_size: 42 };
    println!("sending: {:?}", &request);
    mq.send(request)?;
    Ok(())
}

fn get_prime(mq: message_queue::Client) -> Result<()> {
    let response = mq.receive::<Message>(Tag::Response)?;
    println!("received: {:?}", &response);
    Ok(())
}

fn main() -> Result<()> {
    let mq = message_queue::Client::new("127.0.0.1:8979");

    println!("***Prime numbers client***");
    loop {
        let mut input = String::new();
        stdin().read_line(&mut input);
        
        match input.trim() {
            "ask" => ask_prime(&mq),
            "get" => get_prime(&mq),
            "exit" => break,
            _ => (),
        }
    }

    println!("***Client finished***");
    Ok(())
}
