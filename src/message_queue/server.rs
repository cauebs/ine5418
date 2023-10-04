use anyhow::Result;
use log::info;

use std::{
    collections::HashMap,
    net::{SocketAddr, TcpListener, TcpStream},
    sync::{Arc, Condvar, Mutex, RwLock},
    thread,
};

use super::{Message, MessageKind, MessageOp};

#[derive(Default)]
pub struct Server {
    messages: RwLock<Vec<Message>>,
    waitlist: Mutex<HashMap<SocketAddr, Arc<(Mutex<bool>, Condvar)>>>,
}

impl Server {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn run(&mut self) -> Result<()> {
        let server = TcpListener::bind("0.0.0.0:1101")?;

        thread::scope(|s| {
            for client in server.incoming() {
                s.spawn(|| self.handle_client(client?));
            }
        });

        Ok(())
    }

    fn handle_client(&self, client: TcpStream) -> Result<()> {
        let address = client.peer_addr()?;
        info!("<> Connected to {address}");

        let message_op: MessageOp = bincode::deserialize_from(&client)?;
        info!("<- Received from {address}: {message_op:?}");

        match message_op {
            MessageOp::Send(message) => self.put_message(message),
            MessageOp::Receive(message_kind) => {
                let message = self.get_message(&message_kind, address);
                bincode::serialize_into(&client, &message)?;
                info!("-> Sent to {address}: {message:?}");
            }
        }

        info!(">< Disconnected from {address}");
        Ok(())
    }

    fn put_message(&self, message: Message) {
        if let Message::Response { destination, .. } = message {
            todo!()
        }

        self.messages
            .write()
            .expect("Message queue lock is poisoned")
            .push(message);
    }

    fn try_get_message(&self, message_kind: &MessageKind) -> Option<Message> {
        let index;
        {
            let messages = self
                .messages
                .read()
                .expect("Message queue lock is poisoned");

            index = messages.iter().position(|m| m.kind() == *message_kind);
        }

        if let Some(i) = index {
            let message = self
                .messages
                .write()
                .expect("Message queue lock is poisoned")
                .remove(i);

            return Some(message)
        }

        None
    }

    fn get_message(&self, message_kind: &MessageKind, client: SocketAddr) -> Message {
        loop {
            if let Some(message) = self.try_get_message(message_kind) {
                return message;
            }

            let mut waitlist = self.waitlist.lock().expect("Wait list lock is poisoned");

            let cvar_pair = Arc::new((Mutex::new(false), Condvar::new()));
            waitlist.insert(client, Arc::clone(&cvar_pair));

            let (lock, cvar) = cvar_pair.as_ref();
            let mut message_available = lock.lock().expect("Wait list entry lock is poisoned");
            while !*message_available {
                message_available = cvar.wait(message_available).expect("Wait list entry lock is poisoned");
            }
        }
    }
}
