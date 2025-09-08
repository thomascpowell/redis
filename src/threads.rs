use crate::{command::{execute, parse}, types::DB};

pub fn process(db: &mut DB, string_command: String) -> String {
    match parse(&string_command) {
        Some(cmd) => execute(db, cmd).unwrap_or_else(|err| format!("{:?}", err)),
        _ => format!("unknown error for command: {}", string_command),
    }
}
