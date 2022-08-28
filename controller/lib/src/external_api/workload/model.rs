use serde::{Deserialize, Serialize};

pub enum WorkloadError {
    WorkloadNotFound,
    Etcd(String),
    NameAlreadyExists(String),
    OutOfRange,
    JsonToWorkload(String),
    WorkloadToJson(String),
}

#[derive(Deserialize, Serialize)]

pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Type {
    Container = 0,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ressources {
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ports {
    pub source: i32,
    pub destination: i32,
}
#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct Workload {
    pub id: String,
    pub name: String,
    pub workload_type: Type,
    pub uri: String,
    pub environment: Vec<String>,
    pub resources: Ressources,
    pub ports: Vec<Ports>,
    pub namespace: String,
}

#[derive(Deserialize, Serialize)]
pub struct WorkloadDTO {
    pub name: String,
    pub environment: Vec<String>,
    pub ports: Vec<Ports>,
    pub uri: String,
}
