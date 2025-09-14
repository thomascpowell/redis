use std::time::Instant;

use crate::{
    db::DB,
    types::{Command, Value},
};

pub fn ttl_is_expired(expires_at: Option<Instant>) -> bool {
    expires_at.is_some_and(|ttl| ttl < Instant::now())
}

pub fn add_as_int(db: &mut DB, key: &str, operand: i64) -> Option<i64> {
    let mut res: Option<&Value> = db.store.get(key);
    let mut i: i64;
    let mut expires_at = res.and_then(|x| x.expires_at);
    if ttl_is_expired(expires_at) {
        // if ttl is expired, restart at 0
        i = 0;
        res = None;
        expires_at = None;
        db.del_op(key);
    }
    i = match res {
        Some(v) => v.value.parse().ok()?,
        None => 0,
    };
    i += operand;
    db.store.insert(
        key.to_string(),
        Value {
            value: i.to_string(),
            expires_at: expires_at,
        },
    );
    Some(i)
}

pub fn parse(command: &str) -> Option<Command<'_>> {
    let parts: Vec<&str> = basic_tokenize(command)?;
    match parts.as_slice() {
        ["SET", key, val] => Some(Command::Set {
            key: key,
            value: val,
        }),
        ["SETEX", key, val, ttl] => {
            let ttl = ttl.parse::<u64>().ok()?;
            Some(Command::Setex {
                key,
                value: val,
                ttl: ttl,
            })
        }
        ["EXPIRE", key, ttl] => Some(Command::Expire {
            key,
            ttl: ttl.parse::<u64>().ok()?,
        }),
        ["PERSIST", key] => Some(Command::Persist { key: key }),
        ["TTL", key] => Some(Command::TTL { key: key }),
        ["GET", key] => Some(Command::Get { key: key }),
        ["DEL", key] => Some(Command::Del { key: key }),
        ["INCR", key] => Some(Command::Incr { key: key }),
        ["DECR", key] => Some(Command::Decr { key: key }),
        ["PING"] => Some(Command::Ping),
        _ => None,
    }
}

pub fn tokenize(command: &str) -> Option<Vec<&str>> {
    let mut tokens = command.trim().split("\r\n");


    if !tokens.next().is_some_and(|x| x.starts_with("*")) {
        // if the claimed length matches the real length does not matter for my purposes 
        return None
    }

    while let Some(next) = tokens.next() {
        
    }

    None
}

pub fn basic_tokenize(command: &str) -> Option<Vec<&str>> {
    let tokens: Vec<&str> = command.trim().split_whitespace().collect();
    if tokens.is_empty() {
        None
    } else {
        Some(tokens)
    }
}
