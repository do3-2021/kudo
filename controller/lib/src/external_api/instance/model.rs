use std::net::{Ipv4Addr, SocketAddrV4};

use serde::{Deserialize, Serialize};

const DEFAULT_IP_ADRESS: Ipv4Addr = Ipv4Addr::new(10,0,0,1);

#[derive(Debug)]
pub enum InstanceError {
    InstanceNotFound,
    Etcd(String),
    Grpc(String),
    SerdeError(serde_json::Error),
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub r#type: proto::controller::Type,
    pub state: proto::controller::InstanceState,
    pub status_description: String,
    pub num_restarts: i32,
    pub uri: String,
    pub environment: Vec<String>,
    pub resource: Option<Resource>,
    pub ports: Vec<Port>,
    pub ip: SocketAddrV4,
    pub namespace: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Resource {
    pub limit: Option<ResourceSummary>,
    pub usage: Option<ResourceSummary>,
}


#[derive(Deserialize,Serialize)]
pub struct InstanceDTO {
    pub id: String
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ResourceSummary {
    pub cpu: i32,
    pub memory: i32,
    pub disk: i32,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Port {
    pub source: i32,
    pub dest: i32,
}

// Because we don't want to add a From<> to the proto::controller::InstanceState enum
#[allow(clippy::from_over_into)]
impl Into<proto::scheduler::Instance> for Instance {
    fn into(self) -> proto::scheduler::Instance {
        proto::scheduler::Instance {
            id: self.id,
            name: self.name,
            r#type: self.r#type as i32,
            status: self.state as i32,
            environnement: self.environment,
            ip: self.ip.to_string(),
            ports: self
                .ports
                .into_iter()
                .map(|port| proto::scheduler::Port {
                    source: port.source,
                    destination: port.dest,
                })
                .collect(),
            resource: self.resource.map(|resource| proto::scheduler::Resource {
                limit: resource
                    .limit
                    .map(|resource_summary| proto::scheduler::ResourceSummary {
                        cpu: resource_summary.cpu,
                        memory: resource_summary.memory,
                        disk: resource_summary.disk,
                    }),
                usage: resource
                    .usage
                    .map(|resource_summary| proto::scheduler::ResourceSummary {
                        cpu: resource_summary.cpu,
                        memory: resource_summary.memory,
                        disk: resource_summary.disk,
                    }),
            }),
            uri: self.uri,
        }
    }
}

impl From<super::super::workload::model::Workload> for Instance {
    fn from(workload: super::super::workload::model::Workload) -> Self {
        Self {
            id: workload.id,
            name: workload.name,
            r#type: proto::controller::Type::Container,
            state: proto::controller::InstanceState::Scheduling,
            status_description: "".to_string(),
            num_restarts: 0,
            uri: workload.uri,
            environment: workload.environment,
            namespace: "default".to_string(),
            resource: Some(Resource {
                limit: Some(ResourceSummary {
                    cpu: workload.resources.cpu,
                    memory: workload.resources.memory,
                    disk: workload.resources.disk,
                }),
                usage: None,
            }),
            ports: workload
                .ports
                .into_iter()
                .map(|port| Port {
                    source: port.source,
                    dest: port.destination,
                })
                .collect(),
            ip: SocketAddrV4::new(DEFAULT_IP_ADRESS, 0)
        }
    }
}

fn generate_ip() -> std::net::Ipv4Addr {
    std::net::Ipv4Addr::new(0, 0, 0, 0)
}
