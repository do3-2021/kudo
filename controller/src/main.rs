use controller_lib::external_api;
use controller_lib::internal_api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let grpc_server_addr: String = "0.0.0.0:50051".to_string();
    let grpc_server_num_workers: usize = 1;

    let http_server_addr: String = "0.0.0.0:3000".to_string();
    let http_server_num_workers: usize = 1;

    // Init Logger
    env_logger::init();

    // gRPC Server
    internal_api::interface::InternalAPIInterface::new(grpc_server_addr, grpc_server_num_workers)
        .await;

    // HTTP Server
    external_api::interface::ExternalAPIInterface::new(http_server_addr, http_server_num_workers)
        .await;

    Ok(())
}
