# INE 5418 - Trabalho 1
Formado pelos alunos:
  - André William Régis
  - Cauê Baasch de Souza 
  - Matheus Roque

## Message queue:
Para rodar a fila de mensagens:

```
[RUST_LOG=debug] cargo run --bin mq <bind_addr>:<port> [max-queued-per-client]
```

## Producer:
Para rodar o produtor de números primos:

```
[RUST_LOG=debug] cargo run --bin producer <mq_addr>:<mq_port>
```

## Consumer:
Para rodar o consumidor de números primos:

```
[RUST_LOG=debug] cargo run --bin consumer <mq_addr>:<mq_port>
```

## Webserver:
Para rodar o servidor web:

```
[RUST_LOG=debug] cargo run --bin ws <ws_addr>:<ws_port> <mq_addr>:<mq_port>
```
