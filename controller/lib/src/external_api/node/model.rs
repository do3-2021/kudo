use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::external_api::generic::model::{APIResponse, APIResponseMetadata};
use crate::external_api::instance::model::Instance;

#[derive(Deserialize, Serialize)]
pub struct NodeDTO {
    pub id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]

pub struct ResourceSummary {
    pub cpu: i64,
    pub memory: i64,
    pub disk: i64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Resource {
    pub usage: ResourceSummary,
    pub limit: ResourceSummary,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum NodeState {
    REGISTERING = 0,
    REGISTERED = 1,
    UNREGISTERING = 2,
    UNREGISTERED = 3,
    FAILING = 4,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct NodeStatus {
    pub id: String,
    pub state: NodeState,
    pub status_description: String,
    pub resource: Resource,
    pub instances: Vec<Instance>,
}

#[derive(Deserialize, Serialize)]
pub struct NodeVector {
    pub nodes: Vec<NodeStatus>,
}
impl NodeVector {
    pub fn new(nodes: Vec<NodeStatus>) -> NodeVector {
        NodeVector { nodes }
    }
    pub fn to_http(self) -> HttpResponse {
        HttpResponse::Ok().json(APIResponse {
            data: self,
            metadata: APIResponseMetadata::default(),
        })
    }
}
