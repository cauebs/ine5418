use anyhow::Result;

use super::{Message, MessageOp, StampedMessage};

use std::{
    io,
    net::{TcpStream, ToSocketAddrs},
};

pub struct Client<A: ToSocketAddrs> {
    server_addr: A,
}

impl<A: ToSocketAddrs> Client<A> {
    pub fn new(server_addr: A) -> Self {
        Self { server_addr }
    }

    fn connect(&self) -> io::Result<TcpStream> {
        TcpStream::connect(&self.server_addr)
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
