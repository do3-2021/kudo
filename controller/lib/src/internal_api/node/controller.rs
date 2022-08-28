
use log::info;
use tonic::{Request, Response, Status, Streaming};

use proto::controller::NodeStatus;

use proto::controller::node_service_server::NodeService;

use super::service::update_node_status;

#[derive(Debug, Default)]
pub struct NodeController {}

#[tonic::async_trait]
impl NodeService for NodeController {
    async fn update_node_status(
        &self,
        request: Request<Streaming<NodeStatus>>,
    ) -> Result<Response<()>, Status> {
        let remote_address = request.remote_addr().unwrap();

        info!(
            "{} \"update_node_status\" streaming initiated",
            remote_address.clone()
        );

        let mut stream = request.into_inner();

        while let Some(node_status) = stream.message().await.unwrap() {
            info!(
                "{} \"update_node_status\" received chunk",
                remote_address.clone()
            );
            update_node_status(node_status).unwrap();
        }

        info!(
            "{} \"update_node_status\" streaming closed",
            remote_address.clone()
        );

        Ok(Response::new(()))
    }
}