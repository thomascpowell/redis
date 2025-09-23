use std::sync::mpsc;

use redis::types::{JobRequest, JobResponse, RESPValue};

pub fn get_nil_res() -> String {
    RESPValue::Nil.to_resp()
}

pub fn get_bulk_res(value: &str) -> String {
    RESPValue::BulkString(value.to_string()).to_resp()
}


pub fn get_int_res(value: i64) -> String {
    RESPValue::Integer(value).to_resp()
}


pub fn get_job_request(command: &String) -> JobRequest {
    // dummy channel
    let (test_tx, _) = mpsc::channel::<JobResponse>();
    // turn command into tokens
    // this simulates what the io loop does
    let tokens: Vec<String> = command
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();

    JobRequest {
        tokens: tokens,
        respond_to: test_tx,
    }
}
