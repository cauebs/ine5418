use anyhow::Result;
use uuid::Uuid;

use std::{
    collections::{HashMap, VecDeque},
    io,
    net::{TcpListener, TcpStream, ToSocketAddrs},
    sync::Mutex,
    thread::{self, Thread},
};

use super::{Message, Operation, OperationResult, StampedMessage};

type Waitlist = VecDeque<(Uuid, Thread)>;

#[derive(Default)]
pub struct Server<M: Message> {
    messages: Mutex<VecDeque<StampedMessage<M>>>,
    waitlists: Mutex<HashMap<M::Tag, Waitlist>>,
}

impl<M: Message> Server<M> {
    pub fn new() -> Self {
        Self {
            messages: Mutex::new(VecDeque::new()),
            waitlists: Mutex::new(HashMap::new()),
        }
    }

    pub fn run<A: ToSocketAddrs>(&mut self, bind_addrs: A) -> Result<()> {
        let server = TcpListener::bind(&bind_addrs)?;
        log::info!("Message queue server listening at {}", server.local_addr()?);

        thread::scope(|s| {
            for client in server.incoming() {
                s.spawn(|| {
                    self.handle_client(client)
                        .map_err(|e| log::error!(">< Error while handling client connection: {e}"))
                });
            }
        });

        Ok(())
    }

    fn handle_client(&self, client: io::Result<TcpStream>) -> Result<()> {
        let client = client.map_err(|e| {
            log::error!("!! Failed to connect to client: {e}");
            e
        })?;

        let address = client.peer_addr()?;
        log::debug!("<> Connected to {address}");

        let operation = bincode::deserialize_from(&client)?;
        log::info!("<- Received from {address}: {operation:?}");

        match operation {
            Operation::Register => {
                let id = Uuid::new_v4();
                let result = OperationResult::Ok(id);
                bincode::serialize_into(client, &result)?;
                log::info!("-> Sent to {address}: {result:?}");
            }

            Operation::Send(client_id, message) => {
                self.put_message(StampedMessage {
                    sender: client_id,
                    inner: message,
                });
                let result = OperationResult::Ok(());
                bincode::serialize_into(client, &result)?;
                log::info!("-> Sent to {address}: {result:?}");
            }
            Operation::Receive(client_id, tag) => {
                let message = self.get_message(&tag, client_id);
                let result = OperationResult::Ok(&message);
                match bincode::serialize_into(&client, &result) {
                    Err(_) => {
                        log::warn!(">< Client disconnected while waiting for message: {address}");
                        self.put_message(message);
                        return Ok(());
                    }
                    Ok(_) => log::info!("-> Sent to {address}: {result:?}"),
                }
            }
        }

        log::debug!(">< Disconnected from {address}");
        Ok(())
    }

    fn put_message(&self, message: StampedMessage<M>) {
        let tag = message.inner.tag().clone();
        let recipient = message.inner.recipient();
        {
            self.messages.lock().unwrap().push_back(message);
        }
        self.notify_next_in_waitlist(&tag, recipient);
    }

    fn notify_next_in_waitlist(&self, tag: &M::Tag, recipient: Option<Uuid>) {
        let mut waitlists = self.waitlists.lock().unwrap();
        let Some(waitlist_for_tag) = waitlists.get_mut(tag) else {
            return;
        };

        let waiting = match recipient {
            Some(recipient) => waitlist_for_tag
                .iter()
                .position(|(addr, _thread)| *addr == recipient)
                .and_then(|i| waitlist_for_tag.remove(i)),
            None => waitlist_for_tag.pop_front(),
        };

        if let Some((_addr, thread)) = waiting {
            thread.unpark();
        }
    }

    fn try_get_message(&self, tag: &M::Tag, client_id: Uuid) -> Option<StampedMessage<M>> {
        let mut messages = self.messages.lock().unwrap();
        let first_matching_tag_and_recipient = messages
            .iter()
            .position(|m| m.inner.tag() == *tag && m.inner.recipient() == Some(client_id));

        let mut index = first_matching_tag_and_recipient;

        if index.is_none() {
            let first_matching_tag = messages.iter().position(|m| m.inner.tag() == *tag);
            index = first_matching_tag;
        }

        index.and_then(|i| messages.remove(i))
    }

    fn get_message(&self, tag: &M::Tag, client_id: Uuid) -> StampedMessage<M> {
        loop {
            if let Some(message) = self.try_get_message(tag, client_id) {
                return message;
            }

            self.join_waitlist(tag, client_id);
        }
    }

    fn join_waitlist(&self, tag: &M::Tag, client_id: Uuid) {
        {
            let mut waitlists = self.waitlists.lock().unwrap();
            let waitlist_for_tag = waitlists.entry(tag.clone()).or_default();
            waitlist_for_tag.push_back((client_id, thread::current()));
        }

        thread::park();
    }
}
