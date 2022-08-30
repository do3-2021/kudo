use super::model::Instance;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum InstanceFilterServiceError {
    #[error("Offset out of range")]
    OffsetOutOfRange,
}

/// `FilterService` is a struct that can be used as a service in the WorkloadService.
pub struct InstanceFilterService {}

impl InstanceFilterService {
    pub fn new() -> Self {
        InstanceFilterService {}
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

    pub fn limit(&mut self, instances: &Vec<Instance>, mut limit: u32) -> Vec<Instance> {
        if limit > instances.len() as u32 {
            limit = instances.len() as u32;
        }
        instances[0..limit as usize].to_vec()
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
        instances: &Vec<Instance>,
        offset: u32,
    ) -> Result<Vec<Instance>, InstanceFilterServiceError> {
        if offset > instances.len() as u32 {
            return Err(InstanceFilterServiceError::OffsetOutOfRange);
        }
        Ok(instances[offset as usize..].to_vec())
    }
}
