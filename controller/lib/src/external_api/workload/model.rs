use serde::{Deserialize, Serialize};

pub enum WorkloadError {
    WorkloadNotFound,
    Etcd(String),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Type {
    CONTAINER,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ressources {
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
}
#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct Workload {
    pub id: String,
    pub name: String,
    pub workload_type: Type,
    pub uri: String,
    pub environment: Vec<String>,
    pub resources: Ressources,
    pub ports: Vec<String>,
}
#[derive(Deserialize)]
pub struct WorkloadDTO {
    pub name: String,
    pub environment: Vec<String>,
    pub ports: Vec<String>,
    pub uri: String,
}
