use crate::{
    snapshot,
    types::{Command, JobRequest, JobResponse, RESPValue, Value},
    utils::{exists, get_full_path},
};
use std::{
    fs,
    time::{Duration, Instant},
};

pub struct DB {
    pub store: std::collections::HashMap<String, Value>,
}

/**
* Public DB methods
* */
impl DB {
    pub fn new() -> Self {
        DB {
            store: std::collections::HashMap::new(),
        }
    }

    pub fn restore_or_new(path: &str) -> DB {
        let full_path = get_full_path(&path);
        let res = snapshot::deserialize(&full_path);
        match res {
            Some(db) => {
                println!("restored db");
                return db;
            }
            None => {
                println!("no cache found at: {}", full_path);
                DB::new()
            }
        }
    }

    pub fn process(&mut self, job: &JobRequest) -> JobResponse {
        let value = match parse(&job.tokens) {
            Some(cmd) => self.execute(cmd).to_resp(),
            _ => RESPValue::Err("unknown error for command".to_string()).to_resp(),
        };
        // this is returned to maintain unit testibility
        // worker sends value over mpsc channel
        JobResponse { value: value }
    }

    fn execute(&mut self, command: Command) -> RESPValue {
        match command {
            Command::Ping => RESPValue::Simple("PONG".to_string()),
            Command::Setex { key, value, ttl } => {
                self.setex_op(key.to_string(), value.to_string(), ttl);
                RESPValue::Simple("OK".to_string())
            }
            Command::Set { key, value } => {
                self.set_op(key.to_string(), value.to_string());
                RESPValue::Simple("OK".to_string())
            }
            Command::Del { key } => {
                self.del_op(key);
                RESPValue::Integer(1)
            }
            Command::Get { key } => match self.get_op(key) {
                Some(v) => RESPValue::BulkString(v),
                None => RESPValue::Nil,
            },
            Command::Incr { key } => match self.incr_op(key) {
                Some(v) => RESPValue::Integer(v),
                None => RESPValue::Err("WRONGTYPE".to_string()),
            },
            Command::Decr { key } => match self.decr_op(key) {
                Some(v) => RESPValue::Integer(v),
                None => RESPValue::Err("WRONGTYPE".to_string()),
            },
            Command::Expire { key, ttl } => {
                let v = self.expire_op(key, ttl);
                RESPValue::Integer(v)
            }
            Command::Persist { key } => {
                let v = self.persist_op(key);
                RESPValue::Integer(v)
            }
            Command::TTL { key } => {
                let v = self.ttl_op(key);
                RESPValue::Integer(v)
            }
            Command::Append { key, value } => {
                let v = self.append_op(key, value);
                RESPValue::Integer(v)
            }
            _ => RESPValue::Err("not implemented".to_string()),
        }
    }
}

/**
* Redis Operations
* */
impl DB {
    fn set_op(&mut self, key: String, value: String) {
        let entry = Value {
            value: value,
            expires_at: None,
        };
        self.store.insert(key, entry);
    }

    fn setex_op(&mut self, key: String, value: String, ttl: u64) {
        let entry = Value {
            value: value,
            expires_at: Some(Instant::now() + Duration::from_secs(ttl)),
        };
        self.store.insert(key, entry);
    }

    fn del_op(&mut self, key: &str) {
        self.store.remove_entry(key);
    }

    fn append_op(&mut self, key: &str, addition: &str) -> i64 {
        let new_value: String;
        let new_expires_at: Option<Instant>;
        match self.store.get(key) {
            Some(e) if ttl_is_expired(e.expires_at) => {
                self.del_op(key);
                new_value = addition.to_string();
                new_expires_at = None;
            }
            Some(e) => {
                new_value = e.value.clone() + addition;
                new_expires_at = e.expires_at;
            }
            None => {
                new_value = addition.to_string();
                new_expires_at = None;
            }
        };
        let entry = Value {
            value: new_value,
            expires_at: new_expires_at,
        };
        let len = entry.value.len() as i64;
        self.store.insert(key.to_string(), entry);
        return len;
    }

    fn get_op(&mut self, key: &str) -> Option<String> {
        let entry = self.store.get(key)?;
        if ttl_is_expired(entry.expires_at) {
            self.del_op(key);
            return None;
        }
        return Some(entry.value.clone());
    }

    fn decr_op(&mut self, key: &str) -> Option<i64> {
        add_as_int(self, key, -1)
    }

    fn incr_op(&mut self, key: &str) -> Option<i64> {
        add_as_int(self, key, 1)
    }

    fn expire_op(&mut self, key: &str, ttl: u64) -> i64 {
        match self.store.get(key) {
            Some(entry) if !ttl_is_expired(entry.expires_at) => {
                let val = Value {
                    value: entry.value.clone(),
                    expires_at: Some(Instant::now() + Duration::from_secs(ttl)),
                };
                self.store.insert(key.to_string(), val);
                1
            }
            Some(_) => {
                // key exists but expired
                self.del_op(key);
                0
            }
            None => 0,
        }
    }

    fn persist_op(&mut self, key: &str) -> i64 {
        match self.store.get(key) {
            Some(entry) if !ttl_is_expired(entry.expires_at) => {
                let val = Value {
                    value: entry.value.clone(),
                    expires_at: None,
                };
                self.store.insert(key.to_string(), val);
                1
            }
            Some(_) => {
                self.del_op(key);
                0
            }
            _ => 0,
        }
    }

    fn ttl_op(&mut self, key: &str) -> i64 {
        match self.store.get(key) {
            Some(val) if val.expires_at.is_none() => -1,
            Some(val) if ttl_is_expired(val.expires_at) => {
                self.store.remove(key);
                -2
            }
            Some(val) => {
                let now = Instant::now();
                let remaining: Duration = val.expires_at.unwrap() - now;
                remaining.as_secs() as i64
            }
            None => -2,
        }
    }
}

/**
* Helpers
* */

fn ttl_is_expired(expires_at: Option<Instant>) -> bool {
    expires_at.is_some_and(|ttl| ttl < Instant::now())
}

fn add_as_int(db: &mut DB, key: &str, operand: i64) -> Option<i64> {
    let mut res: Option<&Value> = db.store.get(key);
    let mut i: i64;
    let mut expires_at = res.and_then(|x| x.expires_at);
    if ttl_is_expired(expires_at) {
        // if ttl is expired, restart at 0
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

fn parse(tokens: &Vec<String>) -> Option<Command<'_>> {
    let tokens_ref: Vec<&str> = tokens.iter().map(|s| s.as_str()).collect();
    let cmd = tokens_ref[0].to_ascii_uppercase();
    match tokens_ref.as_slice() {
        [_, key, val] if cmd == "SET" => Some(Command::Set {
            key: key,
            value: val,
        }),
        [_, key, ttl, val] if cmd == "SETEX" => {
            let ttl = ttl.parse::<u64>().ok()?;
            Some(Command::Setex {
                key,
                value: val,
                ttl: ttl,
            })
        }
        [_, key, ttl] if cmd == "EXPIRE" => Some(Command::Expire {
            key,
            ttl: ttl.parse::<u64>().ok()?,
        }),
        [_, key, value] if cmd == "APPEND" => Some(Command::Append { key, value }),
        [_, key] if cmd == "PERSIST" => Some(Command::Persist { key }),
        [_, key] if cmd == "TTL" => Some(Command::TTL { key }),
        [_, key] if cmd == "GET" => Some(Command::Get { key }),
        [_, key] if cmd == "DEL" => Some(Command::Del { key }),
        [_, key] if cmd == "INCR" => Some(Command::Incr { key }),
        [_, key] if cmd == "DECR" => Some(Command::Decr { key }),
        [_] if cmd == "PING" => Some(Command::Ping),
        _ => None,
    }
}
