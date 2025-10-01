use std::{fs, sync::mpsc};

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

pub fn exists(full_path: &String) -> bool {
    fs::exists(&full_path).ok().unwrap()
}

pub fn get_temp_full_path(filename: &str) -> String {
    let temp_dir = std::env::temp_dir().join("redis_temp");
    fs::create_dir_all(&temp_dir).ok();
    temp_dir.join(filename).to_string_lossy().to_string()
}

