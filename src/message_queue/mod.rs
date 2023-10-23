use serde::{de::DeserializeOwned, Deserialize, Serialize};

use std::{fmt::Debug, net::IpAddr, hash::Hash};

mod client;
mod server;

pub use client::Client;
pub use server::Server;

pub trait Message: Serialize + DeserializeOwned + Debug + Send + Sync {
    type Tag: Serialize + DeserializeOwned + Clone + Debug + Send + PartialEq + Eq + Hash;
    fn tag(&self) -> Self::Tag;
    fn recipient(&self) -> Option<IpAddr>;
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageOp<M: Message> {
    #[serde(bound = "M: Message")]
    Send(M),
    Receive(M::Tag),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StampedMessage<M: Message> {
    pub sender: IpAddr,
    #[serde(bound = "M: Message")]
    pub inner: M,
}
