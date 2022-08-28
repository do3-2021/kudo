use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use super::super::workload::model::Workload;
use super::super::workload::service::WorkloadService;
use super::model::InstanceDTO;
use super::service;
use actix_web::http::StatusCode;
use actix_web::{body, web, HttpResponse, Responder, Scope};
use tokio::sync::Mutex;
struct InstanceController {}

impl InstanceController {
    // pub async get_instance(instance_id: web::Path<String>) -> impl Responder {
    //   let mut instance_service = service::InstanceService::new().await;
    // }

    fn getEtcdAddress() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)
    }

    pub async fn put_instance(
        namespace: web::Path<String>,
        body: web::Json<InstanceDTO>,
    ) -> impl Responder {
        let mut instance_service = service::InstanceService::new("0.0.0.0:50051").await;
        let mut workload_service = WorkloadService::new(&InstanceController::getEtcdAddress())
            .await
            .map_err(|err| {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error creating workload service")
            })
            .unwrap();
        let mut id = body.into_inner().id;

        //Check if instance exists, otherwise check workloads
        match instance_service.get_instance(&id, &namespace).await  {
            Ok(instance) => {
                match super::service::InstanceService::retrieve_and_start_instance(
                    Arc::new(Mutex::new(instance_service)),
                    instance,
                )
                .await
                {
                    Ok(_) => return HttpResponse::build(StatusCode::CREATED)
                        .body("Instance creating and starting..."),
                    Err(_) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                        .body("Internal Server Error"),
                }
            },
            Err(_) => {
                match workload_service.get_workload(&id, &namespace).await {
                    Ok(workload) => {
                        match super::service::InstanceService::retrieve_and_start_instance_from_workload(
                            Arc::new(Mutex::new(instance_service)),
                            &id,
                        )
                        .await
                        {
                            Ok(_) => return HttpResponse::build(StatusCode::CREATED)
                                .body("Instance creating..."),
                            Err(_) => return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                                .body("Internal Server Error"),
                        }
                    },
                    Err(_) => return HttpResponse::build(StatusCode::NOT_FOUND)
                        .body("Instance not found"),
                }
            }
        }
    }
    pub async fn delete_instance(
        namespace: web::Path<String>,
        body: web::Json<InstanceDTO>,
    ) -> impl Responder {
        let mut instance_service = service::InstanceService::new("0.0.0.0:50051").await;
        let id = body.into_inner().id;

        match instance_service.get_instance(&id, &namespace).await {
            Ok(instance) => match instance_service.delete_instance(instance).await {
                Ok(_) => return HttpResponse::build(StatusCode::OK).body("Instance deleted"),
                Err(_) => {
                    return HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                        .body("Internal Server Error")
                }
            },
            Err(_) => return HttpResponse::build(StatusCode::NOT_FOUND).body("Instance not found"),
        }

        // match workload_service
        //     .get_workload(instance_dto.id.as_str(), &namespace)
        //     .await
        // {
        //     Ok(_) => match instance_service.get_instance(instance_dto.id.as_str(), &namespace).await {
        //         Ok(instance) => match instance_service.delete_instance(instance).await {
        //             Ok(_) => HttpResponse::build(StatusCode::OK).body("Instance deleted"),
        //             Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
        //                 .body("Internal Server Error"),
        //         },
        //         Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).body("Instance not found"),
        //     },
        //     Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).body("Workload not found"),
        // }
    }

    pub async fn patch_instance(
        namespace: web::Path<String>,
        body: web::Json<InstanceDTO>,
    ) -> impl Responder {
        let mut instance_service = service::InstanceService::new("0.0.0.0:50051").await;
        let instance_dto: InstanceDTO = body.into_inner();
        match instance_service
            .get_instance(instance_dto.id.as_str(), &namespace)
            .await
        {
            Ok(instance) => match instance_service.delete_instance(instance.clone()).await {
                Ok(_) => {
                    match super::service::InstanceService::retrieve_and_start_instance(
                        Arc::new(Mutex::new(instance_service)),
                        instance,
                    )
                    .await
                    {
                        Ok(_) => HttpResponse::build(StatusCode::CREATED)
                            .body("Instance creating and starting..."),
                        Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                            .body("Internal Server Error"),
                    }
                }
                Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
            Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).body("Instance not found"),
        }
    }

    pub async fn get_instance(
        namespace: web::Path<String>,
        body: web::Json<InstanceDTO>,
    ) -> impl Responder {
        let mut instance_service = service::InstanceService::new("0.0.0.0:20051").await;
        let mut workload_service = WorkloadService::new(&InstanceController::getEtcdAddress())
            .await
            .map_err(|err| {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Error creating workload service")
            })
            .unwrap();
        let instance_dto = body.into_inner();
        match workload_service
            .get_workload(instance_dto.id.as_str(), &namespace)
            .await
        {
            Ok(_) => match instance_service
                .get_instance(instance_dto.id.as_str(), &namespace)
                .await
            {
                Ok(instance) => match serde_json::to_string(&instance) {
                    Ok(instance_str) => HttpResponse::build(StatusCode::OK).body(instance_str),
                    Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                        .body("Internal Server Error"),
                },
                Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
            Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).body("Instance not found"),
        }
    }
}

pub fn get_services() -> Scope {
    web::scope("/instance")
        .service(
            web::resource("/{namespace}/{instance_id}")
                .route(web::delete().to(InstanceController::delete_instance))
                .route(web::get().to(InstanceController::get_instance))
                .route(web::patch().to(InstanceController::patch_instance)),
        )
        .service(
            web::resource("/{namespace}").route(web::put().to(InstanceController::put_instance)), // .route(web::get().to(WorkloadController::get_all_instances)),
        )
}
