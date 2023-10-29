use anyhow::Result;
use distribuida::{message_queue::Server, PrimesMessage};

fn main() -> Result<()> {
    env_logger::init();

    let mut args = std::env::args().skip(1);
    let addrs = args
        .next()
        .expect("Use: mq <bind_addr>:<port> [max-queued-per-client]");

    let max_queued_per_client = args.next().map(|s| {
        s.parse()
            .expect("Expected max-queued-per-client to be a number")
    });

    Server::<PrimesMessage>::new()
        .with_throttling(max_queued_per_client)
        .run(addrs)?;

    Ok(())
}
