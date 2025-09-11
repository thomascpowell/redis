use crate::{
    JobRequest, JobResponse,
    types::{Command, RESPValue, Value},
};
use std::error::Error;
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

    fn set_op(&mut self, key: String, value: String, ttl: Option<u64>) {
        let expires_at = match ttl {
            Some(secs) => Some(Instant::now() + Duration::from_secs(secs)),
            None => None,
        };
        let entry = Value { value, expires_at };
        self.store.insert(key, entry);
    }

    fn del_op(&mut self, key: &str) {
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

    fn execute(&mut self, command: Command) -> RESPValue {
        match command {
            Command::Set { key, value, ttl } => {
                self.set_op(key.to_string(), value.to_string(), ttl);
                RESPValue::Simple("OK".to_string())
            }
            Command::Del { key } => {
                self.del_op(key);
                RESPValue::Integer(1)
            }
            Command::Get { key } => match self.get_op(key) {
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
            _ => RESPValue::Err("not implemented".to_string()),
        }
    }
}

fn ttl_is_expired(expires_at: Option<Instant>) -> bool {
    expires_at.is_some_and(|ttl| ttl < Instant::now())
}

// follows redis' rules for coersion
// error if coersion fails
fn add_as_int(db: &mut DB, key: &str, operand: i64) -> Option<i64> {
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

fn parse(command: &str) -> Option<Command<'_>> {
    // currently only supports space delineated
    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["SET", key, val] => Some(Command::Set {
            key: key,
            value: val,
            ttl: None,
        }),
        ["SET", key, val, ttl] => {
            let ttl = ttl.parse::<u64>().ok()?;
            Some(Command::Set {
                key,
                value: val,
                ttl: Some(ttl),
            })
        }
        ["GET", key] => Some(Command::Get { key: key }),
        ["DEL", key] => Some(Command::Del { key: key }),
        ["INCR", key] => Some(Command::Incr { key: key }),
        ["DECR", key] => Some(Command::Decr { key: key }),
        _ => None,
    }
}
