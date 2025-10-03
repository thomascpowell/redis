# "Redis"
A Redis clone written in Rust. Works with existing Redis clients such as `redis-cli` and `go-redis`.

[![tests](https://github.com/thomascpowell/redis/actions/workflows/test.yml/badge.svg)](https://github.com/thomascpowell/redis/actions/workflows/test.yml)

## Features
- [RESP](https://redis.io/docs/latest/develop/reference/protocol-spec/) compliant for supported commands
- Handles multiple clients over TCP
- Automatic persistence with binary snapshots
- Written in Rust with no dependencies

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
- This _can_ be a drop-in replacement for Redis (don't)
- Example usage in a fullstack project can be found [here](https://github.com/thomascpowell/drive.git)
- Common commands and args shown below
```sh
# Run (defaults: 127.0.0.1:6379)
cargo run <ADDR>:<PORT>
```

```sh
# Advanced (defaults: /data/cache and 30)
cargo run <ADDR>:<PORT> <PATH> <INTERVAL>
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
