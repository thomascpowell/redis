use crate::types::{DB, Entry, Value};
use std::time::{Duration, Instant};

impl DB {
    pub fn new() -> Self {
        DB {
            store: std::collections::HashMap::new(),
        }
    }

    pub fn set(&mut self, key: String, value: Value, ttl: Option<u64>) {
        let expires_at = match ttl {
            Some(secs) => Some(Instant::now() + Duration::from_secs(secs)),
            None => None,
        };
        let entry = Entry { value, expires_at };
        self.store.insert(key, entry);
    }

    pub fn get_value(&mut self, key: &str) -> Option<&Value> {
        let entry = self.get_entry(key)?;
        if let Some(expiry) = entry.expires_at && expiry < Instant::now(){
            return None
        }
        return Some(&entry.value)
    }

    pub fn get_entry(&mut self, key: &str) -> Option<&mut Entry> {
        self.store.get_mut(key)
    }
}
