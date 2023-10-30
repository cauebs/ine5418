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

## Webserver:
To run the webserver:

```
cargo run --bin ws <ws_addr>:<ws_port> <mq_addr>:<mq_port>
```