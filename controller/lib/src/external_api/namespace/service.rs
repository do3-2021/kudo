use crate::etcd::{EtcdClient, EtcdClientError};
use crate::external_api::generic::filter::FilterService;
use crate::external_api::workload::model::Workload;
use log::trace;
use serde_json;
use std::net::SocketAddr;
use thiserror::Error;

use super::model::{Metadata, Namespace, NamespaceDTO, NamespaceVector};

#[derive(Debug, Error)]
pub enum NamespaceServiceError {
    #[error("Etcd error: {0}")]
    EtcdError(EtcdClientError),
    #[error("Serde error: {0}")]
    SerdeError(serde_json::Error),
    #[error("Namespace {0} not found")]
    NamespaceNotFound(String),
    #[error("Namespace with name {0} already exists")]
    NameAlreadyExist(String),
}

pub struct NamespaceService {
    etcd_service: EtcdClient,
    filter_service: FilterService,
}

impl NamespaceService {
    pub async fn new(etcd_address: &SocketAddr) -> Result<NamespaceService, NamespaceServiceError> {
        let inner = NamespaceService {
            etcd_service: EtcdClient::new(etcd_address.to_string())
                .await
                .map_err(NamespaceServiceError::EtcdError)?,
            filter_service: FilterService::new(),
        };
        Ok(inner)
    }

    pub async fn namespace(
        &mut self,
        namespace_name: &str,
    ) -> Result<Namespace, NamespaceServiceError> {
        let id = self.id(namespace_name);
        match self.etcd_service.get(&id).await {
            Some(namespace) => {
                let namespace: Namespace =
                    serde_json::from_str(&namespace).map_err(NamespaceServiceError::SerdeError)?;

                trace!("Namespace found: {:?}", namespace);
                Ok(namespace)
            }
            None => Err(NamespaceServiceError::NamespaceNotFound(
                namespace_name.to_string(),
            )),
        }
    }

    pub async fn get_all_namespace(&mut self, limit: u32, offset: u32) -> NamespaceVector {
        let mut new_vec: Vec<Namespace> = Vec::new();
        match self.etcd_service.get_all().await {
            Some(namespaces) => {
                for namespace in namespaces {
                    // if namespace deserialize failed , we don't want to throw error , so we just don't add it to the vector
                    if let Ok(namespace) = serde_json::from_str::<Namespace>(&namespace) {
                        new_vec.push(namespace);
                    }
                }
                if offset > 0 {
                    match self.filter_service.offset(&new_vec, offset) {
                        Ok(namespaces) => new_vec = namespaces,
                        Err(_) => return NamespaceVector::new(vec![]),
                    }
                }
                if limit > 0 {
                    new_vec = self.filter_service.limit(&new_vec, limit);
                }

                trace!("Namespaces found: {:?}", new_vec);
                NamespaceVector::new(new_vec)
            }
            None => NamespaceVector::new(vec![]),
        }
    }

    pub async fn create_namespace(
        &mut self,
        namespace_dto: NamespaceDTO,
    ) -> Result<Namespace, NamespaceServiceError> {
        let id = self.id(&namespace_dto.name);
        match self.namespace(&namespace_dto.name).await {
            Ok(namespace) => Err(NamespaceServiceError::NameAlreadyExist(namespace.name)),
            Err(err) => match err {
                NamespaceServiceError::NamespaceNotFound(_) => {
                    let namespace = Namespace {
                        id: id.to_string(),
                        name: namespace_dto.name,
                        metadata: Metadata {},
                    };
                    let json = serde_json::to_string(&namespace)
                        .map_err(NamespaceServiceError::SerdeError)?;
                    self.etcd_service
                        .put(&id, &json)
                        .await
                        .map_err(NamespaceServiceError::EtcdError)?;

                    trace!("Namespace created: {:?}", namespace);
                    Ok(namespace)
                }
                _ => Err(err),
            },
        }
    }

    pub async fn update_namespace(
        &mut self,
        namespace_dto: NamespaceDTO,
        namespace_name: &str,
    ) -> Result<Namespace, NamespaceServiceError> {
        // we get the id before update , and the new id after update
        let new_id = self.id(&namespace_dto.name);
        self.namespace(namespace_name).await?;
        let namespace = Namespace {
            id: new_id.to_string(),
            name: namespace_dto.name,
            metadata: Metadata {},
        };
        let json = serde_json::to_string(&namespace).map_err(NamespaceServiceError::SerdeError)?;
        self.delete_namespace(namespace_name).await;
        self.etcd_service
            .put(&new_id, &json)
            .await
            .map_err(NamespaceServiceError::EtcdError)?;

        trace!("Namespace updated: {:?}", namespace);
        Ok(namespace)
    }

    pub async fn delete_namespace(&mut self, namespace_name: &str) {
        let id = self.id(namespace_name);

        if let Some(workloads) = self.etcd_service.get_all().await {
            for workload in workloads {
                // we remove all workloads in the namespace
                if let Ok(workload) = serde_json::from_str::<Workload>(&workload) {
                    if workload.namespace == namespace_name {
                        self.etcd_service
                            .delete(&format!("{}.{}", namespace_name, workload.name))
                            .await;
                    }
                }
            }
        };
        _ = self.etcd_service.delete(&id).await;
    }
    pub fn id(&mut self, namespace_name: &str) -> String {
        format!("namespace.{}", namespace_name)
    }
}
