use controller_lib::external_api;
use controller_lib::internal_api;
use log::info;

use std::error::Error;

mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Init Logger
    env_logger::init();

    let config: config::KudoControllerConfig = confy::load_path("controller.conf")?;

    info!("Kudo Controller Configuration initialized: {:?}", config);

    info!("Starting Internal API");

    // gRPC Server
    let internal_api = internal_api::interface::InternalAPIInterface::new(
        config.internal_api.grpc_server_addr,
        config.external_api.etcd_address,
        config.internal_api.grpc_client_addr.clone(),
        config.internal_api.grpc_client_connection_max_retries,
        config.internal_api.time_after_node_erased,
    );

    info!("Starting External API");

    // HTTP Server
    let external_api = external_api::interface::ExternalAPIInterface::new(
        config.external_api.http_server_addr,
        config.external_api.http_server_num_workers,
        config.external_api.etcd_address,
        config.internal_api.grpc_client_addr,
        config.internal_api.grpc_client_connection_max_retries,
    );

    let join = tokio::join!(internal_api, external_api);

    join.0?;
    join.1?;

    Ok(())
}
