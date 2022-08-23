use actix_web::middleware::Logger;
use actix_web::{web, App, HttpResponse, HttpServer};
use kudo_controller_lib::external_api;

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let http_server_addr = "0.0.0.0:3000";
    //HTTP Server
    HttpServer::new(move || {
        App::new()
            .route("/health", web::get().to(HttpResponse::Ok))
            .service(external_api::workload::controller::get_services())
            .wrap(Logger::default())
    })
    .bind(http_server_addr)?
    .run()
    .await
    .unwrap();

    Ok(())
}
