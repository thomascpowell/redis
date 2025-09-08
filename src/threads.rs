use crate::command::{execute, parse};

pub fn process(string_command: String) -> String {
    match parse(&string_command) {
        Some(cmd) => execute(cmd).unwrap_or_else(|err| format!("{:?}", err)),
        _ => format!("unknown error for command: {}", string_command),
    }
}
