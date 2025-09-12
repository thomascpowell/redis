# "Redis"

Minimal implementation of a redis-like key-value store. 

[![Tests](https://github.com/thomascpowell/redis/actions/workflows/test.yml/badge.svg)](https://github.com/thomascpowell/redis/actions/workflows/test.yml)

## Features
- Implements 10 Redis commands including `SET(EX)`, `INCR`, and `EXPIRE`
- Output based on [RESP2](https://redis.io/docs/latest/develop/reference/protocol-spec/)
- Handles multiple clients over TCP
- Written in Rust with no external dependencies

## Usage
```sh
# (ADDR and PORT are optional, defaults: 127.0.0.1:6379)

# run
cargo run <ADDR>:<PORT>

# test (locally)
nc <ADDR> <PORT>

# run tests, build
cargo test
cargo build
```
