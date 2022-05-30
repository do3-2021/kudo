use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use kudo_controller_lib::external_api;
use kudo_controller_lib::internal_api;
use proto::controller::controller_service_server::ControllerServiceServer;
use proto::scheduler::node_service_client::NodeServiceClient;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let grpc_client_addr = "http://0.0.0.0:50051";

    let grpc_server_addr = "0.0.0.0:50051";

    let http_server_addr = "0.0.0.0:3000";

    //gRPC Client
    tokio::spawn(async move {
        NodeServiceClient::connect(grpc_client_addr.clone())
            .await
            .unwrap();
    });

    //gRPC Server
    tokio::spawn(async move {
        Server::builder()
            .add_service(ControllerServiceServer::new(
                internal_api::interface::ControllerServerInterface::default(),
            ))
            .serve(grpc_server_addr.clone().parse().unwrap())
            .await
            .unwrap();
    });

    //HTTP Server
    HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(external_api::workload::controller::get_services())
            .wrap(Logger::default())
    })
    .bind(http_server_addr.clone())?
    .run()
    .await
    .unwrap();

    Ok(())
}
