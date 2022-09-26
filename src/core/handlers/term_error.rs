use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum TermOutputError {
    EmptyString,
}

impl fmt::Display for TermOutputError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            TermOutputError::EmptyString => f.write_str("EmptyString"),
        }
    }
}

impl StdError for TermOutputError {
    fn description(&self) -> &str {
        match *self {
            TermOutputError::EmptyString => "Empty String",
        }
    }
}
