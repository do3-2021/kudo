use crate::external_api::generic::model::{APIResponse, APIResponseMetadata, Pagination};
use crate::external_api::interface::ActixAppState;

use super::model::{Namespace, NamespaceDTO};
use super::service::{NamespaceService, NamespaceServiceError};
use actix_web::http::StatusCode;
use actix_web::{web, HttpResponse, Responder, Scope};
use log::{debug, error};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum NamespaceControllerError {
    #[error("NamespaceServiceError: {0}")]
    NamespaceServiceError(NamespaceServiceError),
}

#[allow(clippy::from_over_into)]
impl Into<HttpResponse> for NamespaceControllerError {
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
            NamespaceControllerError::NamespaceServiceError(err) => match err {
                NamespaceServiceError::NamespaceNotFound(name) => {
                    response.metadata.error = Some(format!("Namespace {} not found", name));
                    status_code = StatusCode::NOT_FOUND;
                    debug!("Namespace {} not found", name);
                }
                NamespaceServiceError::NameAlreadyExist(name) => {
                    status_code = StatusCode::CONFLICT;
                    response.metadata.error =
                        Some(format!("Namespace with name {} already exist", name));
                    debug!("Namespace with name {} already exist", name);
                }
                err => {
                    error!("{}", err);
                }
            },
        }

        HttpResponse::build(status_code).json(response)
    }
}

pub struct NamespaceController {}

impl NamespaceController {
    pub fn services(&self) -> Scope {
        web::scope("/namespace")
            .service(
                web::resource("/{namespace_name}")
                    .route(web::delete().to(NamespaceController::delete_namespace))
                    .route(web::get().to(NamespaceController::namespace))
                    .route(web::patch().to(NamespaceController::patch_namespace)),
            )
            .service(
                web::resource("")
                    .route(web::put().to(NamespaceController::put_namespace))
                    .route(web::get().to(NamespaceController::get_all_namespace)),
            )
    }

    pub async fn namespace(
        params: web::Path<String>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut namespace_service = match NamespaceService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return NamespaceControllerError::NamespaceServiceError(err).into(),
        };

        let namespace_name = params.into_inner();

        match namespace_service.namespace(&namespace_name).await {
            Ok(namespace) => HttpResponse::build(StatusCode::OK).json(APIResponse::<Namespace> {
                metadata: APIResponseMetadata::default(),
                data: namespace,
            }),
            Err(err) => NamespaceControllerError::NamespaceServiceError(err).into(),
        }
    }

    pub async fn put_namespace(
        body: web::Json<NamespaceDTO>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut namespace_service = match NamespaceService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return NamespaceControllerError::NamespaceServiceError(err).into(),
        };

        let namespace_dto = body.into_inner();
        match namespace_service.create_namespace(namespace_dto).await {
            Ok(namespace) => HttpResponse::build(StatusCode::OK).json(APIResponse::<Namespace> {
                metadata: APIResponseMetadata::default(),
                data: namespace,
            }),
            Err(err) => NamespaceControllerError::NamespaceServiceError(err).into(),
        }
    }

    pub async fn get_all_namespace(
        pagination: Option<web::Query<Pagination>>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut namespace_service = match NamespaceService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return NamespaceControllerError::NamespaceServiceError(err).into(),
        };

        let namespaces = match pagination {
            Some(pagination) => {
                namespace_service
                    .get_all_namespace(pagination.limit, pagination.offset)
                    .await
            }
            None => namespace_service.get_all_namespace(0, 0).await,
        };

        HttpResponse::build(StatusCode::OK).json(APIResponse::<Vec<Namespace>> {
            metadata: APIResponseMetadata::default(),
            data: namespaces,
        })
    }

    pub async fn patch_namespace(
        params: web::Path<String>,
        body: web::Json<NamespaceDTO>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut namespace_service = match NamespaceService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return NamespaceControllerError::NamespaceServiceError(err).into(),
        };

        let namespace_name = params.into_inner();
        let namespace_dto = body.into_inner();

        match namespace_service
            .update_namespace(namespace_dto, &namespace_name)
            .await
        {
            Ok(namespace) => HttpResponse::build(StatusCode::OK).json(APIResponse::<Namespace> {
                metadata: APIResponseMetadata::default(),
                data: namespace,
            }),
            Err(err) => NamespaceControllerError::NamespaceServiceError(err).into(),
        }
    }

    pub async fn delete_namespace(
        params: web::Path<String>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut namespace_service = match NamespaceService::new(&data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return NamespaceControllerError::NamespaceServiceError(err).into(),
        };

        let namespace_name = params.into_inner();

        namespace_service.delete_namespace(&namespace_name).await;

        HttpResponse::build(StatusCode::NO_CONTENT).json(APIResponse::<()> {
            metadata: APIResponseMetadata {
                message: Some("Namespace successfully deleted".to_string()),
                ..Default::default()
            },
            ..Default::default()
        })
    }
}
