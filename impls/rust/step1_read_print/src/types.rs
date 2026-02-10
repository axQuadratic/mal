use std::fmt::Display;
use std::fmt::Debug;
use std::vec::Vec;

pub trait MalValue: Display + Debug {}

#[derive(Debug)]
pub struct AtomValue(pub String);

impl MalValue for AtomValue {}

impl Display for AtomValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value = &self.0;
        
        write!(f, "{}", value)
    }
}

#[derive(Debug)]
pub struct ListValue(Vec<Box<dyn MalValue>>);

impl MalValue for ListValue {}

impl Display for ListValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ListValue(values) = self;
        let mut string_values = vec![];

        for element in values {
            string_values.push(element.to_string());
        }
        
        write!(f, "({})", string_values.join(" "))
    }
}

impl ListValue {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn push<T: MalValue + 'static>(&mut self, value: T) {
        let mut values = &mut self.0;

        values.push(Box::new(value));
    }
}
