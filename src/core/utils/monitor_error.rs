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
            MonitorError::InvalidMonitorResolution => f.write_str("InvalidResolution"),
            MonitorError::InvalidMonitorRate => f.write_str("InvalidScreenRate"),
        }
    }
}

impl StdError for MonitorError {
    fn description(&self) -> &str {
        match *self {
            MonitorError::InvalidMonitorResolution => "Invalid Resolution",
            MonitorError::InvalidMonitorRate => "Invalid Screen Rate",
        }
    }
}
