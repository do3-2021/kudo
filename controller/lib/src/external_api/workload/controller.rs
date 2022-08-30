use crate::external_api::interface::ActixAppState;
use crate::external_api::namespace::controller::NamespaceControllerError;

use super::model::{Workload, WorkloadDTO};
use super::service::{WorkloadService, WorkloadServiceError};
use crate::external_api::generic::model::{APIResponse, APIResponseMetadata, Pagination};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, Scope};
use log::{debug, error};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WorkloadControllerError {
    #[error("WorkloadServiceError: {0}")]
    WorkloadServiceError(WorkloadServiceError),
}

#[allow(clippy::from_over_into)]
impl Into<HttpResponse> for WorkloadControllerError {
    fn into(self) -> HttpResponse {
        let mut response = APIResponse::<()> {
            metadata: APIResponseMetadata {
                error: Some("Internal Server Error".to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        let mut status_code = StatusCode::INTERNAL_SERVER_ERROR;

        match self {
            WorkloadControllerError::WorkloadServiceError(err) => match err {
                WorkloadServiceError::WorkloadNotFound(name) => {
                    response.metadata.error = Some(format!("Workload {} not found", name));
                    status_code = StatusCode::NOT_FOUND;
                    debug!("Workload {} not found", name);
                }
                WorkloadServiceError::NameAlreadyExist(name) => {
                    status_code = StatusCode::CONFLICT;
                    response.metadata.error =
                        Some(format!("Workload with name {} already exist", name));
                    debug!("Workload with name {} already exist", name);
                }
                WorkloadServiceError::NamespaceServiceError(err) => {
                    return NamespaceControllerError::NamespaceServiceError(err).into()
                }
                err => {
                    error!("{}", err);
                }
            },
        }

        HttpResponse::build(status_code).json(response)
    }
}

pub struct WorkloadController {}
impl WorkloadController {
    pub fn services(&self) -> Scope {
        web::scope("/workload")
            .service(
                web::resource("/{namespace}/{workload_id}")
                    .route(web::delete().to(WorkloadController::delete_workload))
                    .route(web::get().to(WorkloadController::workload))
                    .route(web::patch().to(WorkloadController::patch_workload)),
            )
            .service(
                web::resource("/{namespace}")
                    .route(web::put().to(WorkloadController::put_workload))
                    .route(web::get().to(WorkloadController::get_all_workloads)),
            )
    }

    /// It gets a workload from etcd, and if it exists, check if the namespace is the same.
    ///
    /// # Arguments:
    ///
    /// * `workload_id`: The workload id to get
    /// * `namespace`: The namespace of the workload
    ///
    /// # Returns:
    ///
    /// A Result<String, WorkloadError>

    pub async fn workload(
        params: web::Path<(String, String)>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let (namespace, workload_id) = params.into_inner();

        let mut workload_service = match WorkloadService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return WorkloadControllerError::WorkloadServiceError(err).into(),
        };

        match workload_service
            .get_workload(&workload_id, &namespace)
            .await
        {
            Ok(workload) => {
                debug!(
                    "GET /workload/{}/{}: {:?}",
                    namespace, workload_id, workload
                );
                HttpResponse::build(StatusCode::OK).json(APIResponse::<Workload> {
                    metadata: APIResponseMetadata::default(),
                    data: workload,
                })
            }
            Err(err) => WorkloadControllerError::WorkloadServiceError(err).into(),
        }
    }

    /// `put_workload` is an async function that handle **/workload/\<namespace>** route (PUT)
    /// # Description:
    /// * Create a new workload
    /// # Arguments:
    ///
    /// * `namespace`: web::Path<String> - This is the namespace that the workload will be created in.
    /// * `body`: web::Json<WorkloadDTO> - Contain all information required to create the workload.

    pub async fn put_workload(
        namespace: web::Path<String>,
        body: web::Json<WorkloadDTO>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut workload_service = match WorkloadService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return WorkloadControllerError::WorkloadServiceError(err).into(),
        };

        let workload_dto = body.into_inner();
        match workload_service
            .create_workload(workload_dto, &namespace)
            .await
        {
            Ok(workload) => {
                HttpResponse::build(StatusCode::CREATED).json(APIResponse::<Workload> {
                    data: workload,
                    metadata: APIResponseMetadata::default(),
                })
            }
            Err(err) => WorkloadControllerError::WorkloadServiceError(err).into(),
        }
    }

    /// `get_all_workloads` is an async function that handle **/workload/\<namespace>** route (GET)
    /// # Description:
    /// * Get all workload in the namespace
    /// # Arguments:
    ///
    /// * `namespace`: The namespace of the workloads you want to retrieve.
    /// * `pagination`: Option<web::Query<Pagination>>

    pub async fn get_all_workloads(
        namespace: web::Path<String>,
        pagination: Option<web::Query<Pagination>>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut workload_service = match WorkloadService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return WorkloadControllerError::WorkloadServiceError(err).into(),
        };

        match pagination {
            Some(pagination) => {
                let workloads = workload_service
                    .get_all_workloads(pagination.limit, pagination.offset, &namespace)
                    .await;
                workloads.to_http()
            }
            None => {
                let workloads = workload_service.get_all_workloads(0, 0, &namespace).await;
                workloads.to_http()
            }
        }
    }

    /// `patch_workload` is an asynchronous function that handle **/workload/\<namespace>/<workload_id>** route (PATCH)
    /// # Description:
    /// * Update a workload
    /// # Arguments:
    ///
    /// * `params`: web::Path<(String, String)> - The first Path parameter is the namespace and the second the workload id.
    /// * `body`: web::Json<WorkloadDTO> - Contain all information required to create the workload.

    pub async fn patch_workload(
        params: web::Path<(String, String)>,
        body: web::Json<WorkloadDTO>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut workload_service = match WorkloadService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return WorkloadControllerError::WorkloadServiceError(err).into(),
        };

        let (namespace, workload_id) = params.into_inner();
        let workload_dto = body.into_inner();

        match workload_service
            .update_workload(workload_dto, &workload_id, &namespace)
            .await
        {
            Ok(workload) => {
                debug!(
                    "PATCH /workload/{}/{}: {:?}",
                    namespace, workload_id, workload
                );
                HttpResponse::build(StatusCode::OK).json(APIResponse::<Workload> {
                    data: workload,
                    metadata: APIResponseMetadata::default(),
                })
            }
            Err(err) => WorkloadControllerError::WorkloadServiceError(err).into(),
        }
    }

    /// It deletes a workload from etcd
    ///
    /// # Arguments:
    ///
    /// * `id`: The id of the workload to delete
    /// * `namespace`: The namespace of the workload
    ///
    /// # Returns:
    ///
    /// A Result<(), WorkloadError>

    pub async fn delete_workload(
        params: web::Path<(String, String)>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut workload_service = match WorkloadService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return WorkloadControllerError::WorkloadServiceError(err).into(),
        };

        let (namespace, workload_id) = params.into_inner();

        workload_service
            .delete_workload(&workload_id, &namespace)
            .await;

        HttpResponse::build(StatusCode::NO_CONTENT).json(APIResponse::<()> {
            metadata: APIResponseMetadata {
                message: Some("Workload successfully deleted".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
