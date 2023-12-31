use anyhow::Result;

use distribuida::{message_queue, PrimesMessage, PrimesTag};
use num_bigint::BigUint;

use std::io::{self, stdin, stdout, Write};

fn print_prompt() -> io::Result<()> {
    print!("> ");
    stdout().flush()
}

fn ask_prime(mq: &message_queue::Client, prime_size: u32, pending_request_cont: &mut i32) {
    let request = PrimesMessage::Request { prime_size };
    log::debug!("sending: {:?}", &request);
    match mq.send_with_retry(request) {
        Ok(_) => {
            *pending_request_cont += 1;
        },
        Err(e) => println!("Error while sending request: {e}"),
    }
}

fn get_prime(mq: &message_queue::Client, pending_request_cont: &mut i32) {
    if *pending_request_cont < 1 {
        println!("No pending prime request.");
        return;
    }
    *pending_request_cont -= 1;

    let Ok(response) = mq.receive::<PrimesMessage>(PrimesTag::Response) else {
        println!("Error while receiving response. Try again.");
        return;
    };

    log::debug!("received: {:?}", &response);

    let PrimesMessage::Response { prime, .. } = response.inner else {
        println!("Asked for a Response, but got a Request!");
        return;
    };

    println!("{}", BigUint::from_bytes_le(&prime));
}

fn main() -> Result<()> {
    let server_addrs = std::env::args()
        .nth(1)
        .expect("Use: consumer <mq_addr>:<mq_port>");

    env_logger::init();

    let mq = message_queue::Client::register(server_addrs)
        .expect("Failed to register to message queue server");

    log::info!("Connected to message queue server with id={}", mq.id);
    println!("Use commands 'ask <prime-size>', 'get' and 'exit'");

    let mut pending_request_cont = 0;
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
                ask_prime(&mq, size, &mut pending_request_cont);
            }
            Some(("get", "")) => get_prime(&mq, &mut pending_request_cont),
            Some(("exit", "")) => break,
            _ => println!("{command} is not a valid command"),
        }

        print_prompt()?;
    }

    Ok(())
}
