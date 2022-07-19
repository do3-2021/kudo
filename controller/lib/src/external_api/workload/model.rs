use serde::Deserialize;

pub struct _Workload {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct WorkloadInfo {
    pub name: String,
}
