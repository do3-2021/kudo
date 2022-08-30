use actix_web::HttpResponse;
use serde::{Deserialize, Serialize};

use crate::external_api::generic::model::{APIResponse, APIResponseMetadata};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Metadata {}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Namespace {
    pub id: String,
    pub name: String,
    pub metadata: Metadata,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct NamespaceDTO {
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct NamespaceVector {
    pub namespaces: Vec<Namespace>,
}
impl NamespaceVector {
    pub fn new(namespaces: Vec<Namespace>) -> NamespaceVector {
        NamespaceVector { namespaces }
    }
    pub fn to_http(&self) -> HttpResponse {
        match serde_json::to_string(&self.namespaces) {
            Ok(json) => HttpResponse::Ok().json(APIResponse {
                data: Some(json),
                metadata: APIResponseMetadata::default(),
            }),
            Err(_) => HttpResponse::InternalServerError().json(APIResponse::<()> {
                metadata: APIResponseMetadata {
                    error: Some("Internal Server Error".to_string()),
                    ..Default::default()
                },
                ..Default::default()
            }),
        }
    }
}
