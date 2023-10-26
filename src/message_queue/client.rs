use anyhow::Result;
use uuid::Uuid;

use super::{Message, Operation, StampedMessage};

use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

pub struct Client {
    server_addrs: Vec<SocketAddr>,
    id: Uuid,
}

impl Client {
    pub fn new<A: ToSocketAddrs>(server_addrs: A) -> Result<Self> {
        let server_addrs = server_addrs
            .to_socket_addrs()
            .expect("Expected address in host:port format")
            .collect::<Vec<_>>();

        let server = TcpStream::connect(server_addrs.as_slice())?;
        let contents: Operation<()> = Operation::Register;
        bincode::serialize_into(&server, &contents)?;
        let id = bincode::deserialize_from(&server)?;

        Ok(Self { server_addrs, id })
    }

    pub fn send<M: Message>(&self, message: M) -> Result<()> {
        let server = TcpStream::connect(self.server_addrs.as_slice())?;
        let contents = Operation::Send(self.id, message);
        bincode::serialize_into(&server, &contents)?;
        Ok(())
    }

    pub fn receive<M: Message>(&self, tag: M::Tag) -> Result<StampedMessage<M>> {
        let server = TcpStream::connect(self.server_addrs.as_slice())?;
        let contents: Operation<M> = Operation::Receive(self.id, tag);
        bincode::serialize_into(&server, &contents)?;
        let message = bincode::deserialize_from(&server)?;
        Ok(message)
    }
}
