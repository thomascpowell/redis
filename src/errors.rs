#[derive(Debug)]
// error types associated with the execute function
pub enum ExecuteError {
    UnknownCommand,
    InvalidArgs,
    NotImplmented,
}

