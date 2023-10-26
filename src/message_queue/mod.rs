use serde::{de::DeserializeOwned, Deserialize, Serialize};
use uuid::Uuid;

use std::{fmt::Debug, hash::Hash};

mod client;
mod server;

pub use client::Client;
pub use server::Server;

pub trait Message: Serialize + DeserializeOwned + Debug + Send + Sync {
    type Tag: Serialize + DeserializeOwned + Clone + Debug + Send + PartialEq + Eq + Hash;
    fn tag(&self) -> Self::Tag;
    fn recipient(&self) -> Option<Uuid>;
}

impl Message for () {
    type Tag = ();

    fn tag(&self) -> Self::Tag {
        ()
    }

    fn recipient(&self) -> Option<Uuid> {
        None
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Operation<M: Message> {
    Register,
    #[serde(bound = "M: Message")]
    Send(Uuid, M),
    Receive(Uuid, M::Tag),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StampedMessage<M: Message> {
    pub sender: Uuid,
    #[serde(bound = "M: Message")]
    pub inner: M,
}
