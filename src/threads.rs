use crate::{db::DB, operations::{execute, parse}};

pub fn process(db: &mut DB, string_command: String) -> String {
    match parse(&string_command) {
        Some(cmd) => execute(db, cmd).to_resp(),
        _ => format!("unknown error for command: {}", string_command),
    }
}
