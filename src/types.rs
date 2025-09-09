use std::time::Instant;

pub struct Value {
    pub value: String,
    pub expires_at: Option<Instant>,
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

pub enum RESPValue {
    Simple(String),
    Err(String),
    Integer(i64),
    Boolean(bool),
    Nil,
}

impl RESPValue {
    pub fn to_resp(&self) -> String {
        // TODO: consider writing to a buffer instead of allocating
        match self {
            RESPValue::Simple(s) => format!("+{}\r\n", s), // Generic return value
            RESPValue::Err(e) => format!("-{}\r\n", e),    // Returned if error internally
            RESPValue::Integer(i) => format!(":{}\r\n", i), // Returned after INCR/DECR, etc
            RESPValue::Boolean(b) => format!("#{}\r\n", if *b { "t" } else { "f" }), // Returned after TOGGLE
            RESPValue::Nil => "$-1\r\n".to_string(), // RESP Spec: "due to historical reasons"
        }
    }
}

pub struct JobRequest {
    pub client: usize,
    pub command: String,
}

impl JobRequest {
    pub fn to_response(&self, value: String) -> JobResponse {
        JobResponse {
            client: self.client,
            value: value,
        }
    }
}

pub struct JobResponse {
    pub client: usize,
    pub value: String,
}
