use crate::types::Value;
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

    pub fn set_op(&mut self, key: String, value: String, ttl: Option<u64>) {
        let expires_at = match ttl {
            Some(secs) => Some(Instant::now() + Duration::from_secs(secs)),
            None => None,
        };
        let entry = Value { value, expires_at };
        self.store.insert(key, entry);
    }

    pub fn del_op(&mut self, key: &str) {
        self.store.remove_entry(key);
    }

    pub fn get_op(&mut self, key: &str) -> Option<String> {
        let entry = self.store.get(key)?;

        if ttl_is_expired(entry.expires_at) {
            self.del_op(key);
            return None;
        }

        return Some(entry.value.clone());
    }
}

fn ttl_is_expired(expires_at: Option<Instant>) -> bool {
    expires_at.is_some_and(|ttl| ttl < Instant::now())
}
