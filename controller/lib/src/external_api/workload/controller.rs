use super::{service, model::WorkloadInfo};
use actix_web::{delete, get, patch, put, web, HttpResponse, Responder, Scope};

pub fn get_service() -> Scope {
    web::scope("/workload")
        .service(get_workloads)
        .service(get_workload)
        .service(put_workload)
        .service(patch_workload)
        .service(delete_workload)
}

#[get("/")]
pub async fn get_workloads() -> impl Responder {
    service::get_workloads(&vec!["1".to_string(), "2".to_string(), "3".to_string()]);
    HttpResponse::Ok().body("workload")
}

#[get("/{workload_id}")]
pub async fn get_workload(workload_id: web::Path<String>) -> impl Responder {
    service::get_workloads(&vec![workload_id.to_string()]);
    HttpResponse::Ok().body(format!("workload {}", workload_id))
}

#[put("/")]
pub async fn put_workload(body: web::Json<WorkloadInfo>) -> impl Responder {
    let workload_info = body.into_inner();
    service::create_workload(&workload_info);
    HttpResponse::Ok().body("put_workload")
}

#[patch("/{workload_id}")]
pub async fn patch_workload(workload_id: web::Path<String>, body: web::Json<WorkloadInfo>) -> impl Responder {
    let workload_info = body.into_inner();
    service::update_workload(&workload_id.to_string(), &workload_info);
    HttpResponse::Ok().body(format!("patch_workload {}", workload_id))
}

#[delete("/{workload_id}")]
pub async fn delete_workload(workload_id: web::Path<String>) -> impl Responder {
    service::delete_workload(&workload_id.to_string());
    HttpResponse::Ok().body(format!("delete_workload {}", workload_id))
}
