use crate::types::{Command, JobRequest, JobResponse, RESPValue, Value};
use crate::helpers::{add_as_int, parse, ttl_is_expired};
use std::time::{Duration, Instant};

pub struct DB {
    pub store: std::collections::HashMap<String, Value>,
}

impl DB {
    pub fn new() -> Self {
        DB {
            store: std::collections::HashMap::new(),
        }
    }

    pub fn process(&mut self, job: &JobRequest) -> JobResponse {
        let value = match parse(&job.command) {
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
                // will need to return bulk strings once added
                Some(v) => RESPValue::Simple(v),
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
            _ => RESPValue::Err("not implemented".to_string()),
        }
    }
}

impl DB {
    pub fn set_op(&mut self, key: String, value: String) {
        let entry = Value {
            value: value,
            expires_at: None,
        };
        self.store.insert(key, entry);
    }

    pub fn setex_op(&mut self, key: String, value: String, ttl: u64) {
        let entry = Value {
            value: value,
            expires_at: Some(Instant::now() + Duration::from_secs(ttl)),
        };
        self.store.insert(key, entry);
    }

    pub fn del_op(&mut self, key: &str) {
        self.store.remove_entry(key);
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
