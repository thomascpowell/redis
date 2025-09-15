# "Redis"

A Redis clone written in Rust. Works will existing Redis clients such as `redis-cli`.

[![Tests](https://github.com/thomascpowell/redis/actions/workflows/test.yml/badge.svg)](https://github.com/thomascpowell/redis/actions/workflows/test.yml)

## Features
- [RESP2](https://redis.io/docs/latest/develop/reference/protocol-spec/) compliant for supported commands
- Handles multiple clients over TCP
- Written in Rust with no external dependencies

## Supported Commands
- `SET key value`: Store a string value
- `SETEX key value ttl`: Set key with expiration
- `GET key`: Get a string value
- `DEL key`: Delete a key
- `INCR key`: Increment as an integer
- `DECR key`: Decrement as an integer
- `EXPIRE key ttl`: Add expiration
- `TTL key`: Check remaining TTL
- `PERSIST key`: Remove TTL
- `PING`: Health check


## Usage
```sh
# Run (defaults: 127.0.0.1:6379)
cargo run <ADDR>:<PORT>
```

```sh
# Connect (raw TCP)
nc <ADDR> <PORT>
```

```sh
# Connect (Redis CLI)
redis-cli
```

```sh
# Misc
cargo build
cargo test
```
