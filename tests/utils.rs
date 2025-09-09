use redis::{JobRequest, JobResponse, RESPValue};

pub fn get_nil_res() -> String {
    RESPValue::Nil.to_resp()
}

pub fn get_simple_res(value: &str) -> String {
    RESPValue::Simple(value.to_string()).to_resp()
}

pub fn get_test_job_request(command: &str) -> JobRequest {
    JobRequest {
        client: 0,
        command: command.to_string(),
    }
}

pub fn get_test_job_response(value: &str) -> JobResponse {
    JobResponse {
        client: 0,
        value: value.to_string(),
    }
}

