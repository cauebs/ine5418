use anyhow::Result;

use super::{Message, MessageKind, MessageOp};

use std::{
    io,
    net::{SocketAddr, TcpStream},
};

pub struct Client {
    server_addr: SocketAddr,
}

impl Client {
    pub fn new(server_addr: impl Into<SocketAddr>) -> Self {
        Self {
            server_addr: server_addr.into(),
        }
    }

    fn connect(&self) -> io::Result<TcpStream> {
        TcpStream::connect(self.server_addr)
    }

    pub fn send_message(&self, message: Message) -> Result<()> {
        let server = self.connect()?;
        let contents = MessageOp::Send(message);
        bincode::serialize_into(&server, &contents)?;
        Ok(())
    }

    pub fn receive_message(&self, kind: MessageKind) -> Result<Message> {
        let server = self.connect()?;
        let contents = MessageOp::Receive(kind);
        bincode::serialize_into(&server, &contents)?;
        let message = bincode::deserialize_from(&server)?;
        Ok(message)
    }
}
