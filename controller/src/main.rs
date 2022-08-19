use controller_lib::external_api;
use controller_lib::internal_api;
use controller_lib::internal_etcd;

use std::error::Error;

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Init Logger
    env_logger::init();

    let config: config::KudoControllerConfig = confy::load_path("controller.conf")?;

    // gRPC Server
    internal_api::interface::InternalAPIInterface::new(
        config.internal_api.grpc_server_addr,
        config.internal_api.grpc_server_num_workers,
    )
    .await;

    // HTTP Server
    external_api::interface::ExternalAPIInterface::new(
        config.external_api.http_server_addr,
        config.external_api.http_server_num_workers,
    )
    .await;

    // ETCD Client
    internal_etcd::interface::EtcdInterface::new().await;
    Ok(())
}
