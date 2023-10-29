# INE 5418 - Trabalho 1

## Message queue:
To run the message queue:

```
cargo run --bin mq <bind_addr>:<port> [max-queued-per-client]
```

## Producer:
To run the producer:

```
cargo run --bin producer <mq_addr>:<mq_port>
```

## Consumer:
To run the consumer:

```
cargo run --bin consumer <mq_addr>:<mq_port>
```
