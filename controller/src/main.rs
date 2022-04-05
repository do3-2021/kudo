use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use kudo_controller_lib::external_api;
use kudo_controller_lib::internal_api;
use kudo_controller_lib::{
    controller::controller_service_client::ControllerServiceClient,
    controller::controller_service_server::ControllerServiceServer,
};
use tonic::transport::Server;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let grpc_client_addr = "http://0.0.0.0:50051";

    let grpc_server_addr = "0.0.0.0:50051";

    let http_server_addr = "0.0.0.0:3000";

    //gRPC Client
    tokio::spawn(async move {
        ControllerServiceClient::connect(grpc_client_addr.clone())
            .await
            .unwrap();
    });

    //gRPC Server
    tokio::spawn(async move {
        Server::builder()
            .add_service(ControllerServiceServer::new(
                internal_api::interface::ControllerInterface::default(),
            ))
            .serve(grpc_server_addr.clone().parse().unwrap())
            .await
            .unwrap();
    });

    //HTTP Server
    HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(external_api::workload::controller::get_service())
            .wrap(Logger::default())
    })
    .bind(http_server_addr.clone())?
    .run()
    .await
    .unwrap();

    Ok(())
}
