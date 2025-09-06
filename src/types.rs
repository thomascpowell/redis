use std::{time::Instant};

#[derive(Clone, Debug)]
pub enum Value {
    Bool(bool),
    Int(i32),
    String(String),
}

pub struct Entry {
    pub value: Value,
    pub expires_at: Option<Instant>,
}

pub struct DB {
    pub store: std::collections::HashMap<String, Entry>,
}
