use crate::{types::{Command, RESPValue, Value}, JobRequest, JobResponse};
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

    pub fn process(&mut self, job: JobRequest) -> JobResponse {
        let value = match parse(&job.command) {
            Some(cmd) => self.execute(cmd).to_resp(),
            _ => format!("unknown error for command: {}", job.command),
        };
        job.to_response(value)
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
        }
    }
}

fn ttl_is_expired(expires_at: Option<Instant>) -> bool {
    expires_at.is_some_and(|ttl| ttl < Instant::now())
}

fn parse(command: &str) -> Option<Command<'_>> {
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
        _ => None,
    }
}
