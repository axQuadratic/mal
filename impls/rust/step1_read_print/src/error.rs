use std::fmt;
use std::convert::From;

pub type MalResult<T> = Result<T, MalError>;

#[derive(Debug)]
pub struct MalError(pub String);

impl std::error::Error for MalError {}

impl fmt::Display for MalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let MalError(s) = self;

        write!(f, "{s}")
    }
}

impl From<&str> for MalError {
    fn from(string: &str) -> Self {
        MalError(String::from(string))
    }
}

impl From<String> for MalError {
    fn from(string: String) -> Self {
        MalError(string)
    }
}

// Used by the REPL to determine whether a read error should be treated as a clean exit
// (e.g. Ctrl+[C|D]), or an unexpected failure
pub enum ReadError {
    Interrupt,
    Failure(String)
}
