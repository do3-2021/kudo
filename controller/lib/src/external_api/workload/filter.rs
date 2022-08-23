use super::model::{Workload, WorkloadError};

pub struct FilterService {}

impl FilterService {
    pub fn new() -> Self {
        FilterService {}
    }

    pub fn limit(&mut self, workloads: &Vec<Workload>, mut limit: u32) -> Vec<Workload> {
        if limit > workloads.len() as u32 {
            limit = workloads.len() as u32;
        }
        return workloads[0..limit as usize].to_vec();
    }

    pub fn offset(
        &mut self,
        workloads: &Vec<Workload>,
        offset: u32,
    ) -> Result<Vec<Workload>, WorkloadError> {
        if offset > workloads.len() as u32 {
            return Err(WorkloadError::OutOfRange);
        }
        return Ok(workloads[offset as usize..].to_vec());
    }

    pub fn filter_by_namespace(
        &mut self,
        workloads: &Vec<Workload>,
        namespace: &str,
    ) -> Vec<Workload> {
        let mut new_vec: Vec<Workload> = Vec::new();
        for workload in workloads {
            if workload.namespace == namespace {
                new_vec.push(workload.clone());
            }
        }
        new_vec
    }
}
