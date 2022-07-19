<<<<<<< HEAD
use super::{model::WorkloadDTO, model::WorkloadError, service};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, Scope};

struct WorkloadController {}

impl WorkloadController {
    pub async fn get_workload(workload_id: web::Path<String>) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;
        match workload_service.get_workload(&workload_id).await {
            Ok(workload) => HttpResponse::build(StatusCode::OK).body(workload.clone()),
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => {
                    HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found")
                }
                WorkloadError::Etcd(e) => {
                    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e)
                }
            },
        }
    }

    pub async fn put_workload(body: web::Json<WorkloadDTO>) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

        let workload_dto = body.into_inner();
        match workload_service.create_workload(workload_dto).await {
            Ok(workload) => HttpResponse::build(StatusCode::CREATED).body(workload),
            Err(_) => {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body("Internal Server Error")
            }
        }
    }

    pub async fn get_all_workloads() -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;
        let workloads = workload_service.get_all_workloads().await;
        web::Json(workloads)
    }

    pub async fn patch_workload(
        workload_id: web::Path<String>,
        body: web::Json<WorkloadDTO>,
    ) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;

        let workload_dto = body.into_inner();

        match workload_service
            .update_workload(&workload_id, workload_dto)
            .await
        {
            Ok(workload) => HttpResponse::build(StatusCode::CREATED).body(workload),
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => {
                    HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found")
                }
                WorkloadError::Etcd(e) => {
                    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e)
                }
            },
=======
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
        let workload_dto = body.into_inner();
        match workload_service.create_workload( workload_dto).await {
            Ok(workload) => HttpResponse::build(StatusCode::CREATED).body(workload),
            Err(WorkloadError::WorkloadNotFound) => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
            Err(WorkloadError::Etcd(e)) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e),
        }
    }

    pub async fn get_all_workloads() -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;
        workload_service.get_all_workloads().await;    
        HttpResponse::build(StatusCode::OK).body("ok")
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
>>>>>>> chore : Controller & service for workloads with etcd
        }

    }

    pub async fn delete_workload(workload_id: web::Path<String>) -> impl Responder {
        let mut workload_service = service::WorkloadService::new().await;
<<<<<<< HEAD

        match workload_service.delete_workload(&workload_id).await {
            Ok(_) => HttpResponse::build(StatusCode::NO_CONTENT).body("Remove successfully"),
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => {
                    HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found")
                }
                WorkloadError::Etcd(e) => {
                    HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e)
                }
            },
=======
        match workload_service.delete_workload(&workload_id).await {
            Ok(_) => HttpResponse::build(StatusCode::NO_CONTENT).body("Remove successfully"), 
            Err(e) => match e {
                WorkloadError::WorkloadNotFound => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
                WorkloadError::Etcd(e) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).body(e), 
            }
>>>>>>> chore : Controller & service for workloads with etcd
        }
    }
}

<<<<<<< HEAD
pub fn get_services() -> Scope {
    web::scope("/workload")
        .service(
            web::resource("/{workload_id}")
                .route(web::delete().to(WorkloadController::delete_workload))
                .route(web::get().to(WorkloadController::get_workload))
                .route(web::patch().to(WorkloadController::patch_workload)),
        )
        .service(
            web::resource("/")
                .route(web::put().to(WorkloadController::put_workload))
                .route(web::get().to(WorkloadController::get_all_workloads)),
        )
=======
pub async fn get_services() -> Scope {
    let mut workload_controller = self::WorkloadController::new().await;
    web::scope("/workload")
        .service(web::resource("/{workload_id}").route(web::delete().to(workload_controller.delete_workload))
            .route(web::get().to(workload_controller.get_workload))
            .route(web::patch().to(workload_controller.patch_workload)))
        .service(web::resource("/").route(web::put().to(workload_controller.put_workload))
            .route(web::get().to(workload_controller.get_all_workloads)))                                         
>>>>>>> chore : Controller & service for workloads with etcd
}
#[cfg(test)]
mod tests {
    use super::WorkloadController;
    use crate::external_api::workload::model::Workload;
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test, web, App,
    };
    use serde_json;
    use std::str::from_utf8;

<<<<<<< HEAD
    #[actix_web::test]
    async fn test_put_and_get() {
        // create temp app http for test
        let app = test::init_service(
            App::new().service(
                web::scope("/workload")
                    .service(
                        web::resource("/").route(web::put().to(WorkloadController::put_workload)),
                    )
                    .service(
                        web::resource("/{workload_id}")
                            .route(web::get().to(WorkloadController::get_workload)),
                    ),
            ),
        )
        .await;
        // create put request to insert workload
        let payload = r#"{
            "name":"nginx",
            "environment" : ["NGINX_PROXY=false"],
            "ports": ["80", "443"],
            "uri":"http://localhost"
        }"#;
        let req_put = test::TestRequest::put()
            .uri("/workload/")
            .insert_header(ContentType::json())
            .set_payload(payload)
            .to_request();
        let resp_put = test::call_and_read_body(&app, req_put).await;
        let workload: Workload = serde_json::from_str(from_utf8(&resp_put).unwrap()).unwrap();
        // create get request to get workload width id in url
        let path_with_id = format!("/workload/{}", workload.id);
        let req_get = test::TestRequest::get().uri(&path_with_id).to_request();
        let resp_get = test::call_service(&app, req_get).await;
        assert!(resp_get.status() == StatusCode::OK);
    }

    #[actix_web::test]
    async fn test_put_and_delete() {
        // create temp app http for test
        let app = test::init_service(
            App::new().service(
                web::scope("/workload")
                    .service(
                        web::resource("/").route(web::put().to(WorkloadController::put_workload)),
                    )
                    .service(
                        web::resource("/{workload_id}")
                            .route(web::delete().to(WorkloadController::delete_workload)),
                    ),
            ),
        )
        .await;
        // create put request to insert workload
        let payload = r#"{
            "name":"nginx",
            "environment" : ["NGINX_PROXY=false"],
            "ports": ["80", "443"],
            "uri":"http://localhost"
        }"#;
        let req_put = test::TestRequest::put()
            .uri("/workload/")
            .insert_header(ContentType::json())
            .set_payload(payload)
            .to_request();
        let resp_put = test::call_and_read_body(&app, req_put).await;
        let workload: Workload = serde_json::from_str(from_utf8(&resp_put).unwrap()).unwrap();
        // create get request to get workload width id in url
        let path_with_id = format!("/workload/{}", workload.id);
        let req_delete = test::TestRequest::delete().uri(&path_with_id).to_request();
        let resp_delete = test::call_service(&app, req_delete).await;
        assert!(resp_delete.status() == StatusCode::NO_CONTENT);
    }

    #[actix_web::test]
    async fn test_get_all() {
        // create temp app http for test
        let app = test::init_service(App::new().service(web::scope("/workload").service(
            web::resource("/").route(web::get().to(WorkloadController::get_all_workloads)),
        )))
        .await;
        // create get_all request to get all workloads
        let req_get = test::TestRequest::get().uri("/workload/").to_request();
        let resp_get = test::call_service(&app, req_get).await;
        assert!(resp_get.status() == StatusCode::OK);
    }
}
=======

>>>>>>> chore : Controller & service for workloads with etcd
