use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::external_api::generic::model::{APIResponse, APIResponseMetadata};

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
    pub resources: Ressources,
}
#[derive(Deserialize, Serialize)]
pub struct WorkloadVector {
    pub workloads: Vec<Workload>,
}
impl WorkloadVector {
    pub fn new(workloads: Vec<Workload>) -> WorkloadVector {
        WorkloadVector { workloads }
    }
    pub fn to_http(&self) -> HttpResponse {
        match serde_json::to_string(&self.workloads) {
            Ok(json) => HttpResponse::Ok().json(APIResponse {
                data: Some(json),
                metadata: APIResponseMetadata::default(),
            }),
            Err(_) => HttpResponse::InternalServerError().json(APIResponse::<()> {
                metadata: APIResponseMetadata {
                    error: Some("Internal Server Error".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }),
        }
    }
}
