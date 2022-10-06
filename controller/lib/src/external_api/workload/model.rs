use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct WorkloadDTO {
    pub name: String,
    pub environment: Vec<String>,
    pub ports: Vec<Ports>,
    pub uri: String,
    pub resources: Ressources,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Type {
    Container = 0,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ressources {
    pub cpu: u64,
    pub memory: u64,
    pub disk: u64,
}
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ports {
    pub source: i32,
    pub destination: i32,
}
