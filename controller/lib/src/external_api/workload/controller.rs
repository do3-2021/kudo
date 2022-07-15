use super::{model::WorkloadDTO, service, model::WorkloadError};
use actix_web::{delete, get, patch, put, web, HttpResponse, Responder, Scope,};
use actix_web::http::{StatusCode};
use serde_json; 

pub fn get_services() -> Scope {
    web::scope("/workload")
        .service(get_all_workloads)
        .service(get_workload)
        .service(put_workload)
        //.service(patch_workload)
        //.service(delete_workload)
}



#[get("/")]
pub async fn get_all_workloads() -> impl Responder {
    service::get_all_workloads();
    // return workloads in json format
    HttpResponse::Ok().body(
        serde_json::to_string(&service::get_all_workloads()).unwrap()
    )
}

#[get("/{workload_id}")]
pub async fn get_workload(workload_id: web::Path<String>) -> impl Responder {
    match service::get_workload(&workload_id) {
        Ok(workload) => HttpResponse::build(StatusCode::OK).body(
            serde_json::to_string(&workload).unwrap()
        ),
        Err(e) => match e {
            WorkloadError::WorkloadNotFound => HttpResponse::build(StatusCode::NOT_FOUND)
            .body("Workload not found"),
            _ => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .body("Internal server error"),
        }
    }
}

#[put("/")]
pub async fn put_workload(body: web::Json<WorkloadDTO>) -> impl Responder {
    let workload_dto = body.into_inner();
    service::create_workload( workload_dto.name, &workload_dto.environment, &workload_dto.ports);
    HttpResponse::build(StatusCode::CREATED)
}
/* 
#[patch("/{workload_id}")]
pub async fn patch_workload(
    workload_id: web::Path<String>,
    body: web::Json<WorkloadInfo>,
) -> impl Responder {
    let workload_info = body.into_inner();

}

#[delete("/{workload_id}")]
pub async fn delete_workload(workload_id: web::Path<String>) -> impl Responder {
    service::delete_workload(&workload_id);
    HttpResponse::Ok().body(format!("delete_workload {}", workload_id))
}
*/