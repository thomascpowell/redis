use redis::RESPValue;

pub fn get_nil_res() -> String {
    RESPValue::Nil.to_resp()
}

pub fn get_simple_res(value: &str) -> String {
    RESPValue::Simple(value.to_string()).to_resp()
}

