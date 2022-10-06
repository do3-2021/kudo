use crate::external_api::{
    generic::model::{APIResponse, APIResponseMetadata, Pagination},
    interface::ActixAppState,
};
use actix_web::{http::StatusCode, web, HttpResponse, Responder, Scope};
use thiserror::Error;

use super::service::{NodeService, NodeServiceError};

use log::{debug, error};

#[derive(Debug, Error)]
pub enum NodeControllerError {
    #[error("NodeServiceError: {0}")]
    NodeServiceError(NodeServiceError),
}

#[allow(clippy::from_over_into)]
impl Into<HttpResponse> for NodeControllerError {
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
            NodeControllerError::NodeServiceError(err) => match err {
                NodeServiceError::NodeNotFound(name) => {
                    response.metadata.error = Some(format!("Node {} not found", name));
                    status_code = StatusCode::NOT_FOUND;
                    debug!("Node {} not found", name);
                }
                err => {
                    error!("{}", err);
                }
            },
        }

        HttpResponse::build(status_code).json(response)
    }
}

pub struct NodeController {}

impl NodeController {
    pub fn services(&self) -> Scope {
        web::scope("/node")
            .service(web::resource("").route(web::get().to(NodeController::get_all_nodes)))
            .service(web::resource("/{node_id}").route(web::get().to(NodeController::get_node)))
    }

    /// It gets a node, and return it if it exists
    ///
    /// # Arguments:
    ///
    /// * id`: The node id to get
    ///
    /// # Returns:
    ///
    /// A HttpResponse with the node if it exists, or a 404 error if it doesn't.

    pub async fn get_node(id: web::Path<String>, data: web::Data<ActixAppState>) -> impl Responder {
        let mut node_service = match NodeService::new(data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return NodeControllerError::NodeServiceError(err).into(),
        };

        match node_service.get_node(&id).await {
            Ok(node) => HttpResponse::build(StatusCode::OK).json(APIResponse {
                data: node,
                metadata: APIResponseMetadata::default(),
            }),
            Err(err) => NodeControllerError::NodeServiceError(err).into(),
        }
    }

    /// `get_all_nodes` is an async function that return all nodes from etcd
    /// # Description:
    /// * Get all nodes inside the cluster
    /// # Arguments:
    ///
    /// * `pagination`: The limit and offset used to paginate the vector of nodes
    /// # Returns:
    /// An HTTP response with the vector of nodes
    pub async fn get_all_nodes(
        pagination: Option<web::Query<Pagination>>,
        data: web::Data<ActixAppState>,
    ) -> impl Responder {
        let mut node_service = match NodeService::new(data.etcd_address).await {
            Ok(service) => service,
            Err(err) => return NodeControllerError::NodeServiceError(err).into(),
        };

        let nodes = match pagination {
            Some(pagination) => {
                node_service
                    .get_all_nodes(pagination.limit, pagination.offset)
                    .await
            }
            None => node_service.get_all_nodes(0, 0).await,
        };

        HttpResponse::build(StatusCode::OK).json(APIResponse {
            data: nodes,
            metadata: APIResponseMetadata::default(),
        })
    }
}
