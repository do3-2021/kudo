use log::{debug, error, info};
use std::net::SocketAddr;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::Mutex;
use tonic::{Code, Request, Response, Status, Streaming};

use super::service::NodeService;

#[derive(Debug, Error)]
pub enum NodeControllerError {
    #[error("Node service error: {0}")]
    NodeServiceError(super::service::NodeServiceError),
}

/// Handles gRPC requests from the scheduler for the node service.
///
/// Properties:
///
/// * `node_service`: An instance of the NodeService that will implement the logic.
pub struct NodeController {
    node_service: Arc<Mutex<NodeService>>,
    time_after_node_erased: u64,
}

impl NodeController {
    pub async fn new(
        etcd_address: &SocketAddr,
        grpc_address: &str,
        grpc_client_connection_max_retries: u32,
        time_after_node_erased: u64,
    ) -> Result<Self, NodeControllerError> {
        Ok(NodeController {
            node_service: Arc::new(Mutex::new(
                NodeService::new(
                    etcd_address,
                    grpc_address,
                    grpc_client_connection_max_retries,
                )
                .await
                .map_err(NodeControllerError::NodeServiceError)?,
            )),
            time_after_node_erased,
        })
    }
}

#[tonic::async_trait]
impl proto::controller::node_service_server::NodeService for NodeController {
    /// It receives the stream sent by the scheduler and updates the persistent storage with the new node status
    ///
    /// # Arguments:
    ///
    /// * `request`: The stream of node status updates
    ///
    /// # Returns:
    ///
    /// A Result<Response<()>, Status>
    async fn update_node_status(
        &self,
        request: Request<Streaming<proto::controller::NodeStatus>>,
    ) -> Result<Response<()>, Status> {
        let stream = request.into_inner();

        debug!("Received node status update stream: {:?}", stream);

        let node_id = self
            .node_service
            .clone()
            .lock()
            .await
            .update_node_status(stream, self.time_after_node_erased)
            .await
            .map_err(|err| {
                error!("Error updating node status: {}", err);
                Status::new(
                    Code::Internal,
                    NodeControllerError::NodeServiceError(err).to_string(),
                )
            })?;

        debug!("Node status update stream closed for Node {}", node_id);
        info!("Node {} is now unregistered", node_id);

        Ok(Response::new(()))
    }
}
