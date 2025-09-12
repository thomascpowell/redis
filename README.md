## "Redis"
[![Tests](https://github.com/thomascpowell/redis/actions/workflows/test.yml/badge.svg)](https://github.com/thomascpowell/redis/actions/workflows/test.yml)

### Features:
- Implements 10 Redis commands including as `SET(EX)`, `INCR`, and `EXPIRE`
- Connects over TCP and provides output based on [RESP2](https://redis.io/docs/latest/develop/reference/protocol-spec/)

### Technical Details:
- Written in Rust with no external dependencies
- Communicates with multiple clients over TCP
