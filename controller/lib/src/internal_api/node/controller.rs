use log::{error, info};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Mutex;
use tonic::{Code, Request, Response, Status, Streaming};

use super::model::NodeStatus;
use super::service::NodeService;

#[derive(Debug)]
pub enum NodeControllerError {
    NodeServiceError(super::service::NodeServiceError),
}

pub struct NodeController {
    node_service: Arc<Mutex<NodeService>>,
}

impl NodeController {
    pub async fn new(etcd_address: SocketAddr) -> Result<Self, NodeControllerError> {
        Ok(NodeController {
            node_service: Arc::new(Mutex::new(
                NodeService::new(etcd_address)
                    .await
                    .map_err(NodeControllerError::NodeServiceError)?,
            )),
        })
    }
}

#[tonic::async_trait]
impl proto::controller::node_service_server::NodeService for NodeController {
    async fn update_node_status(
        &self,
        request: Request<Streaming<proto::controller::NodeStatus>>,
    ) -> Result<Response<()>, Status> {
        let remote_address = if let Some(remote_address) = request.remote_addr() {
            remote_address.to_string()
        } else {
            error!("\"update_node_status\" Failed to get remote address");
            "Error getting remote address".to_string()
        };

        info!(
            "{} \"update_node_status\" streaming initiated",
            remote_address.clone()
        );

        let mut stream = request.into_inner();

        while let Some(node_status) = stream.message().await? {
            info!(
                "{} \"update_node_status\" received chunk",
                remote_address.clone()
            );
            self.node_service
                .clone()
                .lock()
                .await
                .update_node_status(NodeStatus::from(node_status))
                .await
                .map_err(|err| {
                    Status::new(
                        Code::Internal,
                        format!("Error updating node status : {}", err),
                    )
                })?;
        }

        info!(
            "{} \"update_node_status\" streaming closed",
            remote_address.clone()
        );

        Ok(Response::new(()))
    }
}
