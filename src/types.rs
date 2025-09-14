use std::{sync::mpsc::Sender, time::Instant};

pub struct Value {
    pub value: String,
    pub expires_at: Option<Instant>,
}

pub enum Command<'a> {
    Ping,
    Set {
        key: &'a str,
        value: &'a str,
    },
    Setex {
        key: &'a str,
        value: &'a str,
        ttl: u64,
    },
    Get {
        key: &'a str,
    },
    Del {
        key: &'a str,
    },
    Incr {
        key: &'a str,
    },
    Decr {
        key: &'a str,
    },
    Expire {
        key: &'a str,
        ttl: u64,
    },
    TTL {
        key: &'a str,
    },
    Persist {
        key: &'a str,
    },
}

pub enum RESPValue {
    Simple(String),
    Err(String),
    Integer(i64),
    Boolean(bool), // specific to RESP3, which i will not be supporting
    Nil,

    BulkString(String),
    // Array(String), // would be a type here, but commands that return this are not supported

}

impl RESPValue {
    pub fn to_resp(&self) -> String {
        // TODO: consider writing to a buffer instead of allocating
        match self {
            RESPValue::Simple(s) => format!("+{}\r\n", s), // Generic return value
            RESPValue::Err(e) => format!("-{}\r\n", e),    // Returned if error internally
            RESPValue::Integer(i) => format!(":{}\r\n", i), // Returned after INCR/DECR, etc
            RESPValue::Boolean(b) => format!("#{}\r\n", if *b { "t" } else { "f" }),
            RESPValue::Nil => "$-1\r\n".to_string(), // RESP Spec: "due to historical reasons"
            RESPValue::BulkString(s) => format!("${}\r\n{}\r\n", s.len(), s),
        }
    }
}

pub struct JobRequest {
    pub command: String,
    pub respond_to: Sender<JobResponse>,
}
pub struct JobResponse {
    // literally just a string wrapper
    // in case i need to add more stuff later
    pub value: String,
}

impl JobRequest {
    pub fn respond(self, value: String) {
        let response = JobResponse { value };
        let _ = self.respond_to.send(response);
    }
}
