use crate::db::DB;
use crate::types::{Command, RESPValue };

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

pub fn execute(db: &mut DB, command: Command) -> RESPValue {
    match command {
        Command::Set { key, value, ttl } => {
            db.set_op(key.to_string(), value.to_string(), ttl);
            RESPValue::Simple("OK".to_string())
        }
        Command::Del { key } => {
            db.del_op(key);
            RESPValue::Integer(1)
        }
        Command::Get { key } => match db.get_op(key) {
            Some(v) => RESPValue::Simple(v),
            None => RESPValue::Nil,
        },
    }
}
