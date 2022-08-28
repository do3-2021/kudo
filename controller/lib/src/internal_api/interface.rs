use std::net::SocketAddr;

use super::node::controller::NodeController;
use log::info;
use proto::controller::node_service_server::NodeServiceServer;
use tonic::transport::Server;

pub struct InternalAPIInterface {}

impl InternalAPIInterface {
    pub async fn new(address: SocketAddr) -> Self {
        info!("Starting gRPC server listening on {}", address);

        tokio::spawn(async move {
            Server::builder()
                .add_service(NodeServiceServer::new(NodeController::default()))
                .serve(address)
                .await
                .unwrap();
        });

        Self {}
    }
}
