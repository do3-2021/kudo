use super::{model::WorkloadDTO, service, model::WorkloadError, };
use actix_web::{ web, HttpResponse, Responder, Scope,};
use actix_web::http::{StatusCode};


struct WorkloadController {
    workload_service: service::WorkloadService,
}

impl WorkloadController {
    pub async fn new() -> Self {
        WorkloadController { 
            workload_service : service::WorkloadService::new().await
        }
    }
    
    pub async fn get_workload(&mut self,workload_id: web::Path<String>) -> impl Responder {
        match self.workload_service.get_workload(&workload_id).await {
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


    pub async fn put_workload(&mut self,body: web::Json<WorkloadDTO>) -> impl Responder {
        let workload_dto = body.into_inner();
        match self.workload_service.create_workload( workload_dto).await {
            Ok(workload) => HttpResponse::build(StatusCode::CREATED).body(workload),
            Err(WorkloadError::WorkloadNotFound) => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
            Err(WorkloadError::Etcd(e)) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e),
        }
    }

    pub async fn get_all_workloads(&mut self) -> impl Responder {
        self.workload_service.get_all_workloads().await;    
        HttpResponse::build(StatusCode::OK).body("ok")
    } 

    pub async fn patch_workload(&mut self,workload_id: web::Path<String>, body: web::Json<WorkloadDTO>) -> impl Responder {

        let workload_dto = body.into_inner();

        match self.workload_service.update_workload(&workload_id, workload_dto).await {
            Ok(workload) => {
                HttpResponse::build(StatusCode::CREATED).body(workload)
            },
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
                WorkloadError::Etcd(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e),
            }
        }

    }

    pub async fn delete_workload(&mut self,workload_id: web::Path<String>) -> impl Responder {
        match self.workload_service.delete_workload(&workload_id).await {
            Ok(_) => HttpResponse::build(StatusCode::NO_CONTENT).body("Remove successfully"), 
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
                WorkloadError::Etcd(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e), 
            }
        }
    }
}

pub async fn get_services() -> Scope {
    let mut workload_controller = self::WorkloadController::new().await;
    web::scope("/workload")
        .service(web::resource("/{workload_id}").route(web::delete().to(workload_controller.delete_workload))
            .route(web::get().to(workload_controller.get_workload))
            .route(web::patch().to(workload_controller.patch_workload)))
        .service(web::resource("/").route(web::put().to(workload_controller.put_workload))
            .route(web::get().to(workload_controller.get_all_workloads)))                                         
}


