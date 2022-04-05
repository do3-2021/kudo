use serde::Deserialize;

pub struct Workload {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct WorkloadInfo {
    pub name: String,
}
