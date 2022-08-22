use std::net::SocketAddr;

use log::debug;

use crate::etcd::EtcdClient;

use super::model::NodeStatus;

#[derive(Debug)]
pub enum NodeServiceError {
    EtcdError(etcd_client::Error),
    SerdeError(serde_json::Error),
}

impl std::fmt::Display for NodeServiceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NodeServiceError::EtcdError(err) => write!(f, "EtcdError: {}", err),
            NodeServiceError::SerdeError(err) => write!(f, "SerdeError: {}", err),
        }
    }
}

pub struct NodeService {
    etcd_interface: EtcdClient,
}

impl NodeService {
    pub async fn new(etcd_address: SocketAddr) -> Result<Self, NodeServiceError> {
        Ok(NodeService {
            etcd_interface: EtcdClient::new(etcd_address.to_string())
                .await
                .map_err(NodeServiceError::EtcdError)?,
        })
    }

    pub async fn update_node_status(
        &mut self,
        node_status: NodeStatus,
    ) -> Result<(), NodeServiceError> {
        debug!("Updating node status");

        self.etcd_interface
            .put(
                &node_status.id,
                &serde_json::to_string(&node_status).map_err(NodeServiceError::SerdeError)?,
            )
            .await
            .map_err(NodeServiceError::EtcdError)?;

        debug!("Node {} status updated", node_status.id);

        Ok(())
    }
}
