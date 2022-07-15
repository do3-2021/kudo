use std::error::Error;
use std::fmt;


#[derive(Debug)]
pub struct WorkloadManagerError {
    details: String,
    cause: Error 
    // this field will be used to describe the error coming from wheter the workload or its listener
}

impl WorkloadManagerError {
    pub fn new(msg: &str, cause: &Error) -> Self {
        WorkloadManagerError{details: msg.to_string(), cause}
    }
}

impl fmt::Display for WorkloadManagerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}, {}", self.details, self.cause)
    }
}

impl Error for WorkloadManagerError {
    fn description(&self) -> &str {
        format!("{}, {}", &self.details, &self.cause)
    }
}