use std::fmt::Display;
use std::fmt::Debug;

pub trait MalValue: Display + Debug {}

#[derive(Debug)]
pub struct SymbolValue {
    value: String
}

impl MalValue for SymbolValue {}

impl Display for SymbolValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
