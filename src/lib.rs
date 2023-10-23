use std::net::IpAddr;

use serde::{Deserialize, Serialize};

pub mod message_queue;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    Request,
    Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Request { prime_size: u32 },
    Response { recipient: IpAddr, prime: Vec<u8> },
}

impl<'a> message_queue::Message for Message {
    type Tag = Tag;

    fn tag(&self) -> Self::Tag {
        match self {
            Message::Request { .. } => Tag::Request,
            Message::Response { .. } => Tag::Response,
        }
    }

    fn recipient(&self) -> Option<IpAddr> {
        match self {
            Message::Request { .. } => None,
            Message::Response { recipient, .. } => Some(*recipient),
        }
    }
}
