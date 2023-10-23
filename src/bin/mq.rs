use anyhow::Result;
use distribuida::{message_queue::Server, Message};

fn main() -> Result<()> {
    env_logger::init();
    Server::<Message>::new().run()?;
    Ok(())
}
