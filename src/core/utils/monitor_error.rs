use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum MonitorError {
    InvalidMonitorResolution,
    InvalidMonitorRate,
}

impl fmt::Display for MonitorError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ScreenError::InvalidScreenResolution => f.write_str("InvalidResolution"),
            ScreenError::InvalidScreenRate => f.write_str("InvalidScreenRate"),
        }
    }
}

impl StdError for MonitorError {
    fn description(&self) -> &str {
        match *self {
            ScreenError::InvalidScreenResolution => "Invalid Resolution",
            ScreenError::InvalidScreenRate => "Invalid Screen Rate",
        }
    }
}
