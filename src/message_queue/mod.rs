use serde::{Deserialize, Serialize};

use std::{fmt::Debug, net::IpAddr};

pub mod client;
pub mod server;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum MessageKind {
    Request,
    Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageOp {
    Send(Message),
    Receive(MessageKind),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Request {
        prime_size: u32,
    },
    Response {
        destination: IpAddr,
        prime: Vec<u8>,
    },
}

impl Message {
    pub fn kind(&self) -> MessageKind {
        match self {
            Message::Request { .. } => MessageKind::Request,
            Message::Response { .. } => MessageKind::Response,
        }
    }
}
