use tonic::{Request, Response, Status, Streaming};

use crate::controller::NodeStatus;

use super::{
    super::controller::controller_service_server::ControllerService, service::update_node_status,
};

#[derive(Debug, Default)]
pub struct ControllerInterface {}

#[tonic::async_trait]
impl ControllerService for ControllerInterface {
    async fn update_node_status(
        &self,
        request: Request<Streaming<NodeStatus>>,
    ) -> Result<Response<()>, Status> {
        let message = update_node_status().unwrap();

        Ok(Response::new(message))
    }
}
