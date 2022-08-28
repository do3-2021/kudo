use super::workload;
use super::instance;
use super::node;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use log::info;
use std::net::SocketAddr;

pub struct ExternalAPIInterface {}

pub struct ActixAppState {
    pub etcd_address: SocketAddr,
    pub grpc_address: SocketAddr,
}

impl ExternalAPIInterface {
    pub async fn new(address: SocketAddr, num_workers: usize, etcd_address: SocketAddr, grpc_address: SocketAddr) -> Self {
        info!(
            "Starting {} HTTP worker(s) listening on {}",
            num_workers, address
        );

        HttpServer::new(move || {
            App::new()
                .app_data(web::Data::new(ActixAppState { etcd_address, grpc_address }))
                .route("/health", web::get().to(HttpResponse::Ok))
                .service(workload::controller::WorkloadController {}.get_services())
                .service(instance::controller::InstanceController {}.get_services())
                .service(node::controller::NodeController {}.get_services())
                .wrap(Logger::default())
        })
        .workers(num_workers)
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();

        Self {}
    }
}
