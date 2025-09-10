use std::sync::mpsc;

use redis::{JobRequest, JobResponse, RESPValue};

pub fn get_nil_res() -> String {
    RESPValue::Nil.to_resp()
}

pub fn get_simple_res(value: &str) -> String {
    RESPValue::Simple(value.to_string()).to_resp()
}

pub fn get_test_job_request(command: &str) -> JobRequest {
    // dummy channel
    let (test_tx, _) = mpsc::channel::<JobResponse>();
    JobRequest {
        command: command.to_string(),
        respond_to: test_tx,
    }
}

pub fn get_test_job_response(value: &str) -> JobResponse {
    JobResponse {
        value: value.to_string(),
    }
}
