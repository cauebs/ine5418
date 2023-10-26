use anyhow::Result;

use super::{Message, MessageOp, StampedMessage};

use std::{
    io,
    net::{SocketAddr, TcpStream, ToSocketAddrs},
};

pub struct Client {
    server_addrs: Vec<SocketAddr>,
}

impl Client {
    pub fn new<A: ToSocketAddrs>(server_addrs: A) -> Self {
        Self {
            server_addrs: server_addrs.to_socket_addrs().unwrap().collect(),
        }
    }

    fn connect(&self) -> io::Result<TcpStream> {
        TcpStream::connect(self.server_addrs.as_slice())
    }

    pub fn send<M: Message>(&self, message: M) -> Result<()> {
        let server = self.connect()?;
        let contents = MessageOp::Send(message);
        bincode::serialize_into(&server, &contents)?;
        Ok(())
    }

    pub fn receive<M: Message>(&self, tag: M::Tag) -> Result<StampedMessage<M>> {
        let server = self.connect()?;
        let contents: MessageOp<M> = MessageOp::Receive(tag);
        bincode::serialize_into(&server, &contents)?;
        let message = bincode::deserialize_from(&server)?;
        Ok(message)
    }
}
