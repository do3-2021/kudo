use super::model::{Workload, WorkloadError};

/// `FilterService` is a struct that can be used as a service in the WorkloadService.
pub struct FilterService {}

impl FilterService {
    pub fn new() -> Self {
        FilterService {}
    }

    /// It takes a vector of workloads and a limit, and returns a vector of workloads that is limited to the
    /// number of workloads specified by the limit
    ///
    /// # Arguments:
    ///
    /// * `workloads`: A vector of workloads to be limited.
    /// * `limit`: The number of workloads to return.
    ///
    /// # Returns:
    ///
    /// A vector of workloads.
    pub fn limit(&mut self, workloads: &Vec<Workload>, mut limit: u32) -> Vec<Workload> {
        if limit > workloads.len() as u32 {
            limit = workloads.len() as u32;
        }
        workloads[0..limit as usize].to_vec()
    }

    /// "Return a subset of the workloads vector, starting at the offset index."
    ///
    /// The first thing we do is check if the offset is greater than the length of the workloads vector. If
    /// it is, we return an error
    ///
    /// # Arguments:
    ///
    /// * `workloads`: A vector of workloads to be filtered.
    /// * `offset`: The offset to start from.
    ///
    /// # Returns:
    ///
    /// A vector of workloads
    pub fn offset(
        &mut self,
        workloads: &Vec<Workload>,
        offset: u32,
    ) -> Result<Vec<Workload>, WorkloadError> {
        if offset > workloads.len() as u32 {
            return Err(WorkloadError::OutOfRange);
        }
        Ok(workloads[offset as usize..].to_vec())
    }
}
