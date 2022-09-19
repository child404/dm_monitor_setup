use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum LayoutError {
    LayoutNotFound,
    InternalServerError,
}

#[derive(Debug)]
pub enum TermOutputError {
    EmptyString,
}

impl fmt::Display for LayoutError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            LayoutError::LayoutNotFound => f.write_str("LayoutNotFound"),
            LayoutError::InternalServerError => f.write_str("InternalServerError"),
        }
    }
}

impl StdError for LayoutError {
    fn description(&self) -> &str {
        match *self {
            LayoutError::LayoutNotFound => "Layout not found",
            LayoutError::InternalServerError => "Internal Server Error",
        }
    }
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