use proto::controller::NodeState;
use serde::{Deserialize, Serialize};

/// `NodeStatus` is a structure that holds the status of a node registered in the cluster.
///
/// Properties:
///
/// * `id`: The id of the node.
/// * `state`: The current life state of the node.
/// * `status_description`: A text containing details on the status.
/// * `resource`: The resource used by the node.
/// * `instances`: A list of instances that are running on the node.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NodeStatus {
    pub id: String,
    pub state: NodeState,
    pub status_description: String,
    pub resource: Option<Resource>,
    pub instances: Vec<InstanceIdentifier>,
}

/// `Resource` is a structure that holds information about the resource used by the node.
///
/// Properties:
///
/// * `limit`: The maximum amount of resources that this node is allowed to use.
/// * `usage`: The amount of resources currently being used by the node.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Resource {
    pub limit: Option<ResourceSummary>,
    pub usage: Option<ResourceSummary>,
}

/// `ResourceSummary` is a structure that holds information about physical resources.
///
/// Properties:
///
/// * `cpu`: A number in milliCPU that represents a CPU.
/// * `memory`: A number in bytes that represents a memory space.
/// * `disk`: A number in bytes that represents a disk space.
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ResourceSummary {
    pub cpu: u64,
    pub memory: u64,
    pub disk: u64,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct InstanceIdentifier {
    pub id: String,
}

impl PartialEq for InstanceIdentifier {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
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
