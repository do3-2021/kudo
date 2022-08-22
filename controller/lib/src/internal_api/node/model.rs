use proto::controller::NodeState;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct NodeStatus {
    pub id: String,
    pub state: NodeState,
    pub status_description: String,
    pub resource: Option<Resource>,
    pub instances: Vec<InstanceIdentifier>,
}

#[derive(Deserialize, Serialize)]
pub struct Resource {
    pub limit: Option<ResourceSummary>,
    pub usage: Option<ResourceSummary>,
}

#[derive(Deserialize, Serialize)]
pub struct ResourceSummary {
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
}

#[derive(Deserialize, Serialize)]
pub struct InstanceIdentifier {
    pub id: String,
}

impl From<proto::controller::NodeStatus> for NodeStatus {
    fn from(node_status: proto::controller::NodeStatus) -> Self {
        NodeStatus {
            id: node_status.id,
            state: NodeState::from_i32(node_status.state).unwrap_or(NodeState::Failing),
            status_description: node_status.status_description,
            resource: node_status.resource.map(|resource| Resource {
                limit: resource.limit.map(|resource_summary| ResourceSummary {
                    cpu: resource_summary.cpu,
                    memory: resource_summary.memory,
                    disk: resource_summary.disk,
                }),
                usage: resource.usage.map(|resource_summary| ResourceSummary {
                    cpu: resource_summary.cpu,
                    memory: resource_summary.memory,
                    disk: resource_summary.disk,
                }),
            }),
            instances: node_status
                .instances
                .into_iter()
                .map(|instance| InstanceIdentifier { id: instance.id })
                .collect(),
        }
    }
}
