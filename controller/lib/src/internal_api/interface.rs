use std::net::SocketAddr;

use super::node::controller::{NodeController, NodeControllerError};

use log::info;
use proto::controller::node_service_server::NodeServiceServer;
use thiserror::Error;
use tokio::task::JoinError;
use tonic::transport::Server;

#[derive(Debug, Error)]
pub enum InternalAPIInterfaceError {
    #[error("NodeControllerError: {0}")]
    NodeControllerError(NodeControllerError),
    #[error("Thread panicked with error {0}")]
    ThreadError(JoinError),
    #[error("Error while creating gRPC server: {0}")]
    GrpcServeError(tonic::transport::Error),
}

pub struct InternalAPIInterface {}

impl InternalAPIInterface {
    pub async fn new(
        address: SocketAddr,
        etcd_address: SocketAddr,
        grpc_address: String,
        grpc_client_connection_max_retries: u32,
        time_after_node_erased: u64,
    ) -> Result<Self, InternalAPIInterfaceError> {
        info!("Starting gRPC server listening on {}", address);

        tokio::spawn(async move {
            Server::builder()
                .add_service(NodeServiceServer::new(
                    NodeController::new(
                        &etcd_address,
                        &grpc_address,
                        grpc_client_connection_max_retries,
                        time_after_node_erased,
                    )
                    .await
                    .map_err(InternalAPIInterfaceError::NodeControllerError)
                    .unwrap(),
                ))
                .serve(address)
                .await
                .map_err(InternalAPIInterfaceError::GrpcServeError)
                .unwrap();
        })
        .await
        .map_err(InternalAPIInterfaceError::ThreadError)?;

        Ok(Self {})
    }
}
