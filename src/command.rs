use crate::types::{Command, DB, ExecuteError, RESPValue};

pub fn parse(command: &str) -> Option<Command<'_>> {

    let parts: Vec<&str> = command.trim().split_whitespace().collect();
    match parts.as_slice() {
        ["SET", key, val] => Some(Command::Set {
            key: key,
            value: val,
            ttl: None,
        }),
        ["SET", key, val, ttl] => {
            let ttl = ttl.parse::<u64>().ok()?;
            Some(Command::Set {
                key,
                value: val,
                ttl: Some(ttl),
            })
        }
        ["GET", key] => Some(Command::Get { key: key }),
        ["DEL", key] => Some(Command::Del { key: key }),
        _ => None,
    }
}

pub fn execute(db: &mut DB, command: Command) -> Result<String, ExecuteError> {
    // TODO: execute parsed commands here
    // use .to_resp() and return as String

    match command {
        Command::Set { key, value, ttl } => {}
        Command::Del { key } => {}
        Command::Get { key } => {}
    }

    Err(ExecuteError::NotImplmented)
}
