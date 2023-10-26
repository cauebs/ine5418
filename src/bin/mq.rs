use anyhow::Result;
use distribuida::{message_queue::Server, Message};

fn main() -> Result<()> {
    env_logger::init();

    let addrs = std::env::args().skip(1).next().expect("Use: mq <bind_addr>:<port>");
    Server::<Message>::new().run(addrs)?;

    Ok(())
}
