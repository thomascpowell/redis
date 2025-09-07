use crate::types::Command;

pub fn parse(command: &str) -> Option<Command> {
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
