use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::sync::Arc;

use crate::external_api::interface::ActixAppState;

use super::super::workload::service::WorkloadService;
use super::model::InstanceDTO;
use super::service;
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, Scope};
use tokio::sync::Mutex;
pub struct InstanceController {}

impl InstanceController {
    pub fn get_services(&self) -> Scope {
        web::scope("/instance")
            .service(
                web::resource("/{namespace}")
                    .route(web::delete().to(InstanceController::delete_instance))
                    .route(web::get().to(InstanceController::get_instance))
                    .route(web::patch().to(InstanceController::patch_instance))
                    .route(web::put().to(InstanceController::put_instance))
            )
    
    }

    fn get_etcd_address() -> SocketAddr {
        SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0)
    }

    pub async fn put_instance(
        namespace: web::Path<String>,
        body: web::Json<InstanceDTO>,
        data: web::Data<ActixAppState>
    ) -> impl Responder {
        let mut instance_service = service::InstanceService::new(data.grpc_address, data.etcd_address).await
        .map_err(|err| {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Error creating instance service, {:?}", err))
        }).unwrap();
        let mut workload_service = WorkloadService::new(&InstanceController::get_etcd_address())
            .await
            .map_err(|err| {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("Error creating workload service, {:?}", err))
            })
            .unwrap();
        let id = body.into_inner().id;

        //Check if instance exists, otherwise check workloads
        match instance_service.get_instance(&id, &namespace).await  {
            Ok(instance) => {
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
            },
            Err(_) => {
                match workload_service.get_workload(&id, &namespace).await {
                    Ok(_) => {
                        match super::service::InstanceService::retrieve_and_start_instance_from_workload(
                            Arc::new(Mutex::new(instance_service)),
                            &id,
                        )
                        .await
                        {
                            Ok(_) => HttpResponse::build(StatusCode::CREATED)
                                .body("Instance creating..."),
                            Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                                .body("Internal Server Error"),
                        }
                    },
                    Err(_) => HttpResponse::build(StatusCode::NOT_FOUND)
                        .body("Instance not found"),
                }
            }
        }
    }
    pub async fn delete_instance(
        namespace: web::Path<String>,
        body: web::Json<InstanceDTO>,
        data: web::Data<ActixAppState>
    ) -> impl Responder {
        let mut instance_service = service::InstanceService::new(data.grpc_address, data.etcd_address).await
        .map_err(|err| {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Error creating instance service, {:?}", err))
        }).unwrap();
        let id = body.into_inner().id;

        match instance_service.get_instance(&id, &namespace).await {
            Ok(instance) => match instance_service.delete_instance(instance).await {
                Ok(_) => HttpResponse::build(StatusCode::OK).body("Instance deleted"),
                Err(_) => HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body("Internal Server Error"),
            },
            Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).body("Instance not found"),
        }
    }

    pub async fn patch_instance(
        namespace: web::Path<String>,
        body: web::Json<InstanceDTO>,
        data: web::Data<ActixAppState>
    ) -> impl Responder {
        let mut instance_service = service::InstanceService::new(data.grpc_address, data.etcd_address).await
        .map_err(|err| {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Error creating instance service, {:?}", err))
        }).unwrap();
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
        data: web::Data<ActixAppState>,
    ) -> impl Responder {


    // return HttpResponse::build(StatusCode::OK).body("Instance not found");
        
        let mut instance_service = service::InstanceService::new(data.grpc_address, data.etcd_address).await.map_err(|err| {
            HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
            .body(format!("Error creating instance service, {:?}", err))
        }).unwrap();
        let mut workload_service = WorkloadService::new(&InstanceController::get_etcd_address())
            .await
            .map_err(|err| {
                HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR)
                    .body(format!("Error creating workload service {:?}", err))
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
            Err(_) => HttpResponse::build(StatusCode::NOT_FOUND).body("Instance not founded"),
        }
    }
}

