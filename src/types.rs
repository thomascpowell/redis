use std::{
    collections::VecDeque,
    sync::{Condvar, Mutex},
    time::Instant,
};

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

pub enum Command<'a> {
    Set {
        key: &'a str,
        value: &'a str,
        ttl: Option<u64>,
    },
    Get {
        key: &'a str,
    },
    Del {
        key: &'a str,
    },
    // TODO: add bool and int specific commands
    // then add to parse
}

pub struct Queue<T> {
    inner: Mutex<VecDeque<T>>,
    cvar: Condvar,
}

impl<T> Queue<T> {
    pub fn new() -> Self {
        Queue {
            inner: Mutex::new(VecDeque::new()),
            cvar: Condvar::new(),
        }
    }
    pub fn push(&self, item: T) {
        let mut queue = self.inner.lock().unwrap();
        queue.push_back(item);
        self.cvar.notify_one();
    }
    pub fn pop(&self) -> Option<T> {
        let mut queue = self.inner.lock().unwrap();
        queue.pop_front()
    }
    pub fn wait_pop(&self) -> T {
        let mut queue = self.inner.lock().unwrap();
        loop {
            if let Some(item) = queue.pop_front() {
                return item;
            }
            queue = self.cvar.wait(queue).unwrap();
        }
    }
}

#[derive(Debug)]
// error types associated with the execute function
pub enum ExecuteError {
    UnknownCommand,
    InvalidArgs,
    NotImplmented,
}

pub enum RESPValue {
    Simple(String),
    Err(String),
    Integer(i64),
    Boolean(bool),
}

impl RESPValue {
    pub fn to_resp(&self) -> String {
        match self {
            RESPValue::Simple(s) => format!("+{}\r\n", s), // Generic return value
            RESPValue::Err(e) => format!("-{}\r\n", e),    // Returned if error internally
            RESPValue::Integer(i) => format!(":{}\r\n", i), // Returned after INCR/DECR
            RESPValue::Boolean(b) => format!("#{}\r\n", if *b { "t" } else { "f" }), // Returned after toggle
        }
    }
}
