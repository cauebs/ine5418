use anyhow::Result;
use serde::de::DeserializeOwned;
use uuid::Uuid;

use super::{Message, Operation, OperationResult, StampedMessage};
use crate::ExponentialBackoff;

use std::net::{SocketAddr, TcpStream, ToSocketAddrs};

pub struct Client {
    server_addrs: Vec<SocketAddr>,
    pub id: Uuid,
}

fn register(server_addrs: &[SocketAddr]) -> Result<Uuid> {
    let server = TcpStream::connect(server_addrs)?;
    let op: Operation = Operation::Register;
    bincode::serialize_into(&server, &op)?;
    let result: OperationResult<Uuid> = bincode::deserialize_from(&server)?;
    let id = result?;
    Ok(id)
}

impl Client {
    pub fn register<A: ToSocketAddrs>(server_addrs: A) -> Result<Self> {
        let server_addrs = server_addrs
            .to_socket_addrs()
            .expect("Expected address in host:port format")
            .collect::<Vec<_>>();

        let id = register(&server_addrs)?;
        Ok(Self { server_addrs, id })
    }

    fn execute_operation<M: Message, R: DeserializeOwned>(&self, op: &Operation<M>) -> Result<R> {
        let server = TcpStream::connect(self.server_addrs.as_slice())?;
        bincode::serialize_into(&server, &op)?;

        let result: OperationResult<R> = bincode::deserialize_from(&server)?;
        Ok(result?)
    }

    pub fn send<M: Message>(&self, message: M) -> Result<()> {
        let op = Operation::Send(self.id, message);
        self.execute_operation(&op)
    }

    pub fn send_with_retry<M: Message>(&self, message: M) -> Result<()> {
        let op = Operation::Send(self.id, message);

        for wait_duration in ExponentialBackoff::default() {
            let op_result = self.execute_operation(&op);
            if op_result.is_ok() {
                return op_result;
            };

            log::info!(
                "Waiting {} seconds before retrying send operation...",
                wait_duration.as_secs()
            );
        }

        Ok(())
    }

    pub fn receive<M: Message>(&self, tag: M::Tag) -> Result<StampedMessage<M>> {
        let op: Operation<M> = Operation::Receive(self.id, tag);
        self.execute_operation(&op)
    }
}
