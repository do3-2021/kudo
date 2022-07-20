use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct WorkloadError {
    details: String
}

impl WorkloadError {
    pub fn new(msg: &str) -> WorkloadError {
        WorkloadError{details: msg.to_string()}
    }
}

impl fmt::Display for WorkloadError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for WorkloadError {
    fn description(&self) -> &str {
        &self.details
    }
}