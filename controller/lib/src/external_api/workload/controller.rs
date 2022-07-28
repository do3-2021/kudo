use super::{model::WorkloadDTO, service, model::WorkloadError, };
use actix_web::{ web, HttpResponse, Responder, Scope,};
use actix_web::http::{StatusCode};


struct WorkloadController {}

impl WorkloadController {

    pub async fn get_workload(workload_id: web::Path<String>) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;
        match workload_service.get_workload(&workload_id).await {
            Ok(workload) => HttpResponse::build(StatusCode::OK).body(
                workload.clone()
            ),
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => HttpResponse::build(StatusCode::NOT_FOUND)
                    .body("Workload not found"),
                WorkloadError::Etcd(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e),
            }
        }
    }

    pub async fn put_workload(body: web::Json<WorkloadDTO>) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

    pub async fn put_workload(body: web::Json<WorkloadDTO>) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

        let workload_dto = body.into_inner();
        match workload_service.create_workload( workload_dto).await {
            Ok(workload) => HttpResponse::build(StatusCode::CREATED).body(workload),
            Err(WorkloadError::WorkloadNotFound) => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
            Err(WorkloadError::Etcd(e)) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e),
        }
    }

    pub async fn get_all_workloads() -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;
        let workloads = workload_service.get_all_workloads().await;
        web::Json(workloads)
    } 

    pub async fn patch_workload(workload_id: web::Path<String>, body: web::Json<WorkloadDTO>) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;


        let workload_dto = body.into_inner();

        match workload_service.update_workload(&workload_id, workload_dto).await {
            Ok(workload) => {
                HttpResponse::build(StatusCode::CREATED).body(workload)
            },
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
                WorkloadError::Etcd(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e),
            }
        }

    }

    pub async fn delete_workload(workload_id: web::Path<String>) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

        match workload_service.delete_workload(&workload_id).await {
            Ok(_) => HttpResponse::build(StatusCode::NO_CONTENT).body("Remove successfully"), 
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
                WorkloadError::Etcd(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e), 
            }
        }
    }
}

pub fn get_services() -> Scope {
    web::scope("/workload")
        .service(web::resource("/{workload_id}").route(web::delete().to(WorkloadController::delete_workload))
            .route(web::get().to(WorkloadController::get_workload))
            .route(web::patch().to(WorkloadController::patch_workload)))
        .service(web::resource("/").route(web::put().to(WorkloadController::put_workload))
            .route(web::get().to(WorkloadController::get_all_workloads)))                                         
}