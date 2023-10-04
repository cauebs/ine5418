use distribuida::message_queue;

fn main() -> anyhow::Result<()> {
    env_logger::init();
    message_queue::server::main()
}
