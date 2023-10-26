use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod message_queue;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum Tag {
    Request,
    Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Message {
    Request { prime_size: u32 },
    Response { recipient: Uuid, prime: Vec<u8> },
}

impl message_queue::Message for Message {
    type Tag = Tag;

    fn tag(&self) -> Self::Tag {
        match self {
            Message::Request { .. } => Tag::Request,
            Message::Response { .. } => Tag::Response,
        }
    }

    fn recipient(&self) -> Option<Uuid> {
        match self {
            Message::Request { .. } => None,
            Message::Response { recipient, .. } => Some(*recipient),
        }
    }
}
