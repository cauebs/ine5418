use anyhow::Result;

use std::{
    collections::{HashMap, VecDeque},
    net::{IpAddr, TcpListener, TcpStream},
    sync::Mutex,
    thread::{self, Thread},
};

use super::{Message, MessageOp, StampedMessage};

type Waitlist = VecDeque<(IpAddr, Thread)>;

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

    pub fn run(&mut self) -> Result<()> {
        let server = TcpListener::bind("0.0.0.0:8979")?;

        thread::scope(|s| {
            for client in server.incoming() {
                s.spawn(|| self.handle_client(client?));
            }
        });

        Ok(())
    }

    fn handle_client(&self, client: TcpStream) -> Result<()> {
        let address = client.peer_addr()?;
        log::debug!("<> Connected to {address}");

        let message_op: MessageOp<M> = bincode::deserialize_from(&client)?;
        log::info!("<- Received from {address}: {message_op:?}");

        match message_op {
            MessageOp::Send(message) => self.put_message(StampedMessage {
                sender: address.ip(),
                inner: message,
            }),
            MessageOp::Receive(tag) => {
                let message = self.get_message(&tag, address.ip());
                bincode::serialize_into(&client, &message)?;
                log::info!("-> Sent to {address}: {message:?}");
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

    fn notify_next_in_waitlist(&self, tag: &M::Tag, recipient: Option<IpAddr>) {
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

    fn try_get_message(&self, tag: &M::Tag, client: IpAddr) -> Option<StampedMessage<M>> {
        let mut messages = self.messages.lock().unwrap();
        let first_matching_tag_and_recipient = messages
            .iter()
            .position(|m| m.inner.tag() == *tag && m.inner.recipient() == Some(client));

        let mut index = first_matching_tag_and_recipient;

        if index.is_none() {
            let first_matching_tag = messages.iter().position(|m| m.inner.tag() == *tag);
            index = first_matching_tag;
        }

        index.and_then(|i| messages.remove(i))
    }

    fn get_message(&self, tag: &M::Tag, client: IpAddr) -> StampedMessage<M> {
        loop {
            if let Some(message) = self.try_get_message(tag, client) {
                return message;
            }

            self.join_waitlist(tag, client);
        }
    }

    fn join_waitlist(&self, tag: &M::Tag, client: IpAddr) {
        {
            let mut waitlists = self.waitlists.lock().unwrap();
            let waitlist_for_tag = waitlists.entry(tag.clone()).or_default();
            waitlist_for_tag.push_back((client, thread::current()));
        }

        thread::park();
    }
}
