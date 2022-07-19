use serde::{Deserialize, Serialize};

pub enum WorkloadError {
    WorkloadNotFound,
<<<<<<< HEAD
    Etcd(String),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
=======
	Etcd(String)
}

#[derive(Deserialize,Serialize, Clone, Debug)]
>>>>>>> chore : Controller & service for workloads with etcd
pub enum Type {
    CONTAINER,
}
<<<<<<< HEAD
#[derive(Deserialize, Serialize, Clone, Debug)]
=======
#[derive(Deserialize,Serialize, Clone, Debug)]
>>>>>>> chore : Controller & service for workloads with etcd
pub struct Ressources {
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
}
<<<<<<< HEAD
#[derive(Deserialize, Serialize, Clone, Debug)]
=======
#[derive(Deserialize,Serialize, Clone,Debug)]
#[serde(rename_all = "camelCase")]
>>>>>>> chore : Controller & service for workloads with etcd

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
