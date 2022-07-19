use super::workload;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use log::info;

pub struct ExternalAPIInterface {}

impl ExternalAPIInterface {
    pub async fn new(address: String, num_workers: usize) -> Self {
        info!(
            "Starting {} HTTP worker(s) listening on {}",
            num_workers, address
        );

        HttpServer::new(move || {
            App::new()
                .route("/health", web::get().to(HttpResponse::Ok))
                .service(workload::controller::get_services())
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
