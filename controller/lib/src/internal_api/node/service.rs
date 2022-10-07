use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use log::{info, trace};
use thiserror::Error;
use tokio::sync::Mutex;
use tokio::time;
use tonic::Streaming;

use crate::etcd::{EtcdClient, EtcdClientError};
use crate::external_api::instance::service::InstanceServiceError;
use crate::internal_api::node::model::InstanceIdentifier;

use super::super::super::external_api::instance::{model::Instance, service::InstanceService};
use super::model::NodeStatus;

#[derive(Debug, Error)]
pub enum NodeServiceError {
    #[error("Etcd error: {0}")]
    EtcdError(EtcdClientError),
    #[error("Serde error: {0}")]
    SerdeError(serde_json::Error),
    #[error("Streaming closed: {0}")]
    StreamingClosed(tonic::Status),
    #[error("InstanceServiceError: {0}")]
    InstanceServiceError(InstanceServiceError),
}

pub struct NodeService {
    etcd_interface: EtcdClient,
    instance_service: Arc<Mutex<InstanceService>>,
}

impl NodeService {
    pub async fn new(
        etcd_address: &SocketAddr,
        grpc_address: &str,
        grpc_client_connection_max_retries: u32,
    ) -> Result<Self, NodeServiceError> {
        Ok(NodeService {
            etcd_interface: EtcdClient::new(etcd_address.to_string())
                .await
                .map_err(NodeServiceError::EtcdError)?,
            instance_service: Arc::new(Mutex::new(
                InstanceService::new(
                    grpc_address,
                    etcd_address,
                    grpc_client_connection_max_retries,
                )
                .await
                .map_err(NodeServiceError::InstanceServiceError)?,
            )),
        })
    }

    /// It receives a stream of `NodeStatus` messages from the scheduler, updates the node's status in etcd,
    /// and reschedules any instances that were running on the node if the stream is closed.
    ///
    /// Arguments:
    ///
    /// * `stream`: The stream of messages received from the scheduler.
    /// * `remote_address`: The address of the scheduler gRPC server that is sending the status update.
    ///
    /// Returns:
    ///
    /// A `Result` with an `Ok` value of `()` or an `Err` value of `NodeServiceError`.
    pub async fn update_node_status(
        &mut self,
        mut stream: Streaming<proto::controller::NodeStatus>,
        time_after_node_erased: u64,
    ) -> Result<String, NodeServiceError> {
        let mut last_instances: Vec<Instance> = vec![];
        let mut node_id = String::new();

        while let Some(node_status) = stream
            .message()
            .await
            .map_err(NodeServiceError::StreamingClosed)?
        {
            let node_status = NodeStatus::from(node_status);

            trace!("Node status update: {:?}", node_status);

            self.etcd_interface
                .put(
                    &node_status.id,
                    &serde_json::to_string(&node_status).map_err(NodeServiceError::SerdeError)?,
                )
                .await
                .map_err(NodeServiceError::EtcdError)?;

            let objects = self.etcd_interface.get_all().await.unwrap_or_default();

            for object in objects {
                if let Ok(instance) = serde_json::from_str::<Instance>(&object) {
                    if node_status.instances.contains(&InstanceIdentifier {
                        id: instance.clone().id,
                    }) {
                        last_instances.push(instance);
                    }
                }
            }

            if node_id.is_empty() {
                info!("Node {} is now registered", node_status.id);
            }

            node_id = node_status.id;
        }

        for instance in last_instances {
            InstanceService::schedule_instance(self.instance_service.clone(), instance)
        }

        time::sleep(Duration::from_secs(time_after_node_erased)).await;
        self.etcd_interface.delete(&node_id).await;

        Ok(node_id)
    }
}
