use std::sync::Arc;

use crate::external_api::generic::model::{APIResponse, APIResponseMetadata, Pagination};
use crate::external_api::interface::ActixAppState;
use crate::external_api::workload::controller::WorkloadControllerError;
use crate::external_api::workload::service::WorkloadServiceError;

use super::super::workload::service::WorkloadService;
use super::model::{Instance, InstanceDTO};
use super::service::{InstanceService, InstanceServiceError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, Scope};
use log::{debug, error};
use thiserror::Error;
use tokio::sync::Mutex;

#[derive(Error, Debug)]
pub enum InstanceControllerError {
    #[error("Instance service error: {0}")]
    InstanceServiceError(InstanceServiceError),
    #[error("Workload service error: {0}")]
    WorkloadServiceError(WorkloadServiceError),
}

#[allow(clippy::from_over_into)]
impl Into<HttpResponse> for InstanceControllerError {
    fn into(self) -> HttpResponse {
        let mut response = APIResponse::<()> {
            metadata: APIResponseMetadata {
                error: Some("Internal server error".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

        match self {
            InstanceControllerError::InstanceServiceError(err) => match err {
                InstanceServiceError::WorkloadNotFound(name) => {
                    status_code = StatusCode::NOT_FOUND;
                    response.metadata.error = Some(format!("Workload {} not found", name));
                    debug!("Workload {} not found", name)
                }
                InstanceServiceError::InstanceNotFound(name) => {
                    status_code = StatusCode::NOT_FOUND;
                    response.metadata.error = Some(format!("Instance {} not found", name));
                    debug!("Instance {} not found", name)
                }
                err => {
                    error!("{}", err);
                }
            },
            InstanceControllerError::WorkloadServiceError(e) => {
                return WorkloadControllerError::WorkloadServiceError(e).into()
            }
        }

        HttpResponse::build(status_code).json(response)
    }
}

pub struct InstanceController {}

impl InstanceController {
    pub fn services(&self) -> Scope {
        web::scope("/instance")
            .service(
                web::resource("/{namespace}")
                    .route(web::get().to(InstanceController::get_instances))
                    .route(web::post().to(InstanceController::post_instance)),
            )
            .service(
                web::resource("/{namespace}/{name}")
                    .route(web::delete().to(InstanceController::delete_instance))
                    .route(web::get().to(InstanceController::get_instance)),
            )
    }

    /// `post_instance` is an async function that handle **/instance/\<namespace>** route (POST)
    /// # Description:
    /// * Create and start an instance
    /// # Arguments:
    ///
    /// * `namespace`: web::Path<String> - This is the namespace that the instance will be created in.
    /// * `body`: web::Json<InstanceDTO> - Contains the workload_name to create the instance from it.

    pub async fn post_instance(
        namespace: web::Path<String>,
        body: web::Json<InstanceDTO>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let instance_service =
            match InstanceService::new(&data.grpc_address, &data.etcd_address).await {
                Ok(service) => service,
                Err(err) => {
                    return InstanceControllerError::InstanceServiceError(err).into();
                }
            };

        let mut workload_service = match WorkloadService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => {
                return InstanceControllerError::WorkloadServiceError(err).into();
            }
        };

        match workload_service
            .get_workload(&body.workload_name, &namespace)
            .await
        {
            Ok(workload) => {
                match super::service::InstanceService::retrieve_and_start_instance_from_workload(
                    Arc::new(Mutex::new(instance_service)),
                    &workload.id,
                )
                .await
                {
                    Ok(_) => HttpResponse::build(StatusCode::CREATED).json(APIResponse::<()> {
                        metadata: APIResponseMetadata {
                            message: Some("Instance creating and starting...".to_string()),
                            ..Default::default()
                        },
                        ..Default::default()
                    }),
                    Err(e) => InstanceControllerError::InstanceServiceError(e).into(),
                }
            }
            Err(e) => InstanceControllerError::WorkloadServiceError(e).into(),
        }
    }
    /// It deletes a instance from etcd and grpc.
    ///
    /// # Arguments:
    ///
    /// * `id`: The id of the instance to delete
    /// * `namespace`: The namespace of the workload
    ///
    /// # Returns:
    ///
    /// A Result<(), InstanceError>
    pub async fn delete_instance(
        params: web::Path<(String, String)>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut instance_service =
            match InstanceService::new(&data.grpc_address, &data.etcd_address).await {
                Ok(service) => service,
                Err(err) => {
                    return InstanceControllerError::InstanceServiceError(err).into();
                }
            };

        let (namespace, name) = params.into_inner();
        match instance_service
            .get_instance(format!("{}.{}", namespace, name).as_str())
            .await
        {
            Ok(instance) => match instance_service.delete_instance(instance).await {
                Ok(_) => HttpResponse::build(StatusCode::OK).json(APIResponse::<()> {
                    metadata: APIResponseMetadata {
                        message: Some("Instance deleted".to_string()),
                        ..Default::default()
                    },
                    ..Default::default()
                }),
                Err(err) => InstanceControllerError::InstanceServiceError(err).into(),
            },
            Err(e) => InstanceControllerError::InstanceServiceError(e).into(),
        }
    }

    /// It gets a instance from etcd, and if it exists, check if the namespace is the same.
    ///
    /// # Arguments:
    ///
    /// * `namespace`: The namespace of the instance
    /// * `name`: The workload name to get
    ///
    /// # Returns:
    ///
    /// A Result<String, InstanceError>
    pub async fn get_instance(
        params: web::Path<(String, String)>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut instance_service =
            match InstanceService::new(&data.grpc_address, &data.etcd_address).await {
                Ok(service) => service,
                Err(err) => {
                    return InstanceControllerError::InstanceServiceError(err).into();
                }
            };

        let (namespace, name) = params.into_inner();
        match instance_service
            .get_instance(format!("{}.{}", namespace, name).as_str())
            .await
        {
            Ok(instance) => HttpResponse::build(StatusCode::OK).json(APIResponse::<Instance> {
                data: instance,
                metadata: APIResponseMetadata::default(),
            }),
            Err(e) => InstanceControllerError::InstanceServiceError(e).into(),
        }
    }

    /// `get_instances` is an async function that handle **/instance/\<namespace>** route (GET)
    /// # Description:
    /// * Get all instances in the namespace
    /// # Arguments:
    ///
    /// * `namespace`: The namespace of the instance you want to retrieve.
    /// * `pagination`: Option<web::Query<Pagination>>
    pub async fn get_instances(
        namespace: web::Path<String>,
        pagination: Option<web::Query<Pagination>>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut instance_service =
            match InstanceService::new(&data.grpc_address, &data.etcd_address).await {
                Ok(service) => service,
                Err(err) => {
                    return InstanceControllerError::InstanceServiceError(err).into();
                }
            };

        let instances = match pagination {
            Some(pagination) => {
                instance_service
                    .get_instances(pagination.limit, pagination.offset, &namespace)
                    .await
            }
            None => instance_service.get_instances(0, 0, &namespace).await,
        };

        HttpResponse::build(StatusCode::OK).body(
            serde_json::to_string(&APIResponse::<Vec<Instance>> {
                data: instances,
                metadata: APIResponseMetadata::default(),
            })
            .unwrap_or_default(),
        )
    }
}
