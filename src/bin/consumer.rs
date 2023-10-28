use anyhow::Result;

use distribuida::{message_queue, Message, Tag};
use num_bigint::BigUint;

use std::io::{self, stdin, stdout, Write};

fn print_prompt() -> io::Result<()> {
    print!("> ");
    stdout().flush()
}

fn ask_prime(mq: &message_queue::Client, prime_size: u32) {
    let request = Message::Request { prime_size };
    log::debug!("sending: {:?}", &request);
    let Ok(_) = mq.send(request) else {
        println!("Error while sending request. Try again.");
        return;
    };
}

fn get_prime(mq: &message_queue::Client) {
    let Ok(response) = mq.receive::<Message>(Tag::Response) else {
        println!("Error while receiving response. Try again.");
        return;
    };

    log::debug!("received: {:?}", &response);

    let Message::Response { prime, .. } = response.inner else {
        println!("Asked for a Response, but got a Request!");
        return;
    };

    println!("{}", BigUint::from_bytes_le(&prime));
}

fn main() -> Result<()> {
    let server_addrs = std::env::args()
        .skip(1)
        .next()
        .expect("Use: consumer <mq_addr>:<mq_port>");

    env_logger::init();

    let mq = message_queue::Client::register(server_addrs)
        .expect("Failed to register to message queue server");

    log::info!("Connected to message queue server with id={}", mq.id);
    println!("Use commands 'ask <prime-size>', 'get' and 'exit'");

    print_prompt()?;
    for line in stdin().lines() {
        let line = line?;
        let command = line.trim();

        match command.split_once(' ').or(Some((command, ""))) {
            Some(("ask", n)) => {
                let Ok(size) = n.parse() else {
                    println!("Expected a number for the prime size");
                    print_prompt()?;
                    continue;
                };
                ask_prime(&mq, size);
            }
            Some(("get", "")) => get_prime(&mq),
            Some(("exit", "")) => break,
            _ => println!("{command} is not a valid command"),
        }

        print_prompt()?;
    }

    Ok(())
}
