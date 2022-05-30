use tonic::{Request, Response, Status, Streaming};

use proto::controller::NodeStatus;

use proto::controller::controller_service_server::ControllerService;

use super::service::update_node_status;

#[derive(Debug, Default)]
pub struct ControllerServerInterface {}

#[tonic::async_trait]
impl ControllerService for ControllerServerInterface {
    async fn update_node_status(
        &self,
        request: Request<Streaming<NodeStatus>>,
    ) -> Result<Response<()>, Status> {
        let message = update_node_status().unwrap();

        Ok(Response::new(message))
    }
}
