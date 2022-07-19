use super::node::controller::NodeController;
use log::info;
use proto::controller::controller_service_server::ControllerServiceServer;
use tonic::transport::Server;

pub struct InternalAPIInterface {}

impl InternalAPIInterface {
    pub async fn new(address: String, num_workers: usize) -> Self {
        info!(
            "Starting {} gRPC worker(s) listening on {}",
            num_workers, address
        );

        for _ in 1..num_workers {
            let address = address.clone();
            tokio::spawn(async move {
                Server::builder()
                    .add_service(ControllerServiceServer::new(NodeController::default()))
                    .serve(address.parse().unwrap())
                    .await
                    .unwrap();
            });
        }
        Self {}
    }
}
