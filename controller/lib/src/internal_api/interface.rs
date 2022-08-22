use std::net::SocketAddr;

use super::node::controller::NodeController;
use log::info;
use proto::controller::node_service_server::NodeServiceServer;
use tonic::transport::Server;

#[derive(Debug)]
pub enum NodeInterfaceError {
    NodeControllerError(super::node::controller::NodeControllerError),
    TonicError(tonic::transport::Error),
}

pub struct InternalAPIInterface {}

impl InternalAPIInterface {
    pub async fn new(address: SocketAddr, num_workers: usize, etcd_address: SocketAddr) -> Self {
        info!(
            "Starting {} gRPC worker(s) listening on {}",
            num_workers, address
        );

        for _ in 1..num_workers {
            tokio::spawn(async move {
                Server::builder()
                    .add_service(NodeServiceServer::new(
                        NodeController::new(etcd_address)
                            .await
                            .map_err(NodeInterfaceError::NodeControllerError)
                            .unwrap(),
                    ))
                    .serve(address)
                    .await
                    .map_err(NodeInterfaceError::TonicError)
                    .unwrap()
            });
        }

        InternalAPIInterface {}
    }
}
