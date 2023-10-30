use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod message_queue;
pub mod utils;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PrimesTag {
    Request,
    Response,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PrimesMessage {
    Request { prime_size: u32 },
    Response { recipient: Uuid, prime: Vec<u8> },
}

impl message_queue::Message for PrimesMessage {
    type Tag = PrimesTag;

    fn tag(&self) -> Self::Tag {
        match self {
            PrimesMessage::Request { .. } => PrimesTag::Request,
            PrimesMessage::Response { .. } => PrimesTag::Response,
        }
    }

    fn recipient(&self) -> Option<Uuid> {
        match self {
            PrimesMessage::Request { .. } => None,
            PrimesMessage::Response { recipient, .. } => Some(*recipient),
        }
    }
}
