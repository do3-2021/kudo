use std::net::SocketAddr;

use super::model::{Ressources, Type, Workload, WorkloadDTO, WorkloadVector};
use crate::etcd::{EtcdClient, EtcdClientError};
use crate::external_api::generic::filter::FilterService;
use crate::external_api::namespace::service::{NamespaceService, NamespaceServiceError};
use log::trace;
use serde_json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum WorkloadServiceError {
    #[error("Etcd error: {0}")]
    EtcdError(EtcdClientError),
    #[error("Serde error: {0}")]
    SerdeError(serde_json::Error),
    #[error("Workload {0} not found")]
    WorkloadNotFound(String),
    #[error("Workload with name {0} already exists")]
    NameAlreadyExist(String),
    #[error("Namespace service error: {0}")]
    NamespaceServiceError(NamespaceServiceError),
}

/// `WorkloadService` is a struct that inpired from Controllers Provider Modules architectures. It can be used as a service in the WorkloadController .A service can use other services.
/// Properties:
///
/// * `etcd_service`: This is the service that will be used to interact with etcd.
/// * `filter_service`: This is the service that will be used to filter the workloads.
pub struct WorkloadService {
    etcd_service: EtcdClient,
    filter_service: FilterService,
    namespace_service: NamespaceService,
}

impl WorkloadService {
    pub async fn new(etcd_address: &SocketAddr) -> Result<WorkloadService, WorkloadServiceError> {
        let inner = WorkloadService {
            etcd_service: EtcdClient::new(etcd_address.to_string())
                .await
                .map_err(WorkloadServiceError::EtcdError)?,
            filter_service: FilterService::new(),
            namespace_service: NamespaceService::new(etcd_address)
                .await
                .map_err(WorkloadServiceError::NamespaceServiceError)?,
        };
        Ok(inner)
    }
    pub async fn get_workload(
        &mut self,
        workload_name: &str,
        namespace: &str,
    ) -> Result<Workload, WorkloadServiceError> {
        let id = self.id(workload_name, namespace);
        //check if namespace exists
        self.namespace_service
            .namespace(namespace)
            .await
            .map_err(WorkloadServiceError::NamespaceServiceError)?;

        match self.etcd_service.get(&id).await {
            Some(workload) => {
                let workload: Workload =
                    serde_json::from_str(&workload).map_err(WorkloadServiceError::SerdeError)?;
                if workload.namespace == namespace {
                    trace!("Workload found: {:?}", workload);
                    Ok(workload)
                } else {
                    Err(WorkloadServiceError::WorkloadNotFound(
                        workload_name.to_string(),
                    ))
                }
            }
            None => Err(WorkloadServiceError::WorkloadNotFound(
                workload_name.to_string(),
            )),
        }
    }

    /// This function gets all the workloads from etcd, filters them by namespace and slice the result by limit and offset
    /// If there is an error , the function always return an empty vector
    /// # Arguments:
    ///
    /// * `limit`: The number of workloads to return.
    /// * `offset`: The offset of the workloads to be returned.
    /// * `namespace`: The namespace to filter by.
    ///
    /// # Returns:
    ///
    /// A vector of workloads
    pub async fn get_all_workloads(
        &mut self,
        limit: u32,
        offset: u32,
        namespace: &str,
    ) -> WorkloadVector {
        let mut new_vec: Vec<Workload> = Vec::new();
        match self.etcd_service.get_all().await {
            Some(workloads) => {
                for workload in workloads {
                    // if workload deserialize failed , we don't want to throw error , so we just don't add it to the vector
                    if let Ok(workload) = serde_json::from_str::<Workload>(&workload) {
                        if workload.namespace == namespace {
                            new_vec.push(workload);
                        }
                    }
                }
                if offset > 0 {
                    match self.filter_service.offset(&new_vec, offset) {
                        Ok(workloads) => new_vec = workloads,
                        Err(_) => return WorkloadVector::new(vec![]),
                    }
                }
                if limit > 0 {
                    new_vec = self.filter_service.limit(&new_vec, limit);
                }

                trace!("Workloads found: {:?}", new_vec);
                WorkloadVector::new(new_vec)
            }
            None => WorkloadVector::new(vec![]),
        }
    }

    /// It creates a new workload in etcd
    ///
    /// # Arguments:
    ///
    /// * `workload_dto`: WorkloadDTO containt the workload data
    /// * `namespace`: The namespace of the workload
    ///
    /// # Returns:
    ///
    /// A workload.
    pub async fn create_workload(
        &mut self,
        workload_dto: WorkloadDTO,
        namespace: &str,
    ) -> Result<Workload, WorkloadServiceError> {
        let new_id = self.id(&workload_dto.name, namespace);
        match self.get_workload(&workload_dto.name, namespace).await {
            Ok(workload) => Err(WorkloadServiceError::NameAlreadyExist(workload.name)),
            Err(err) => match err {
                WorkloadServiceError::WorkloadNotFound(_) => {
                    let workload = Workload {
                        id: new_id.to_string(),
                        name: workload_dto.name,
                        workload_type: Type::Container,
                        uri: workload_dto.uri,
                        environment: workload_dto.environment,
                        resources: Ressources {
                            cpu: workload_dto.resources.cpu,
                            memory: workload_dto.resources.memory,
                            disk: workload_dto.resources.disk,
                        },
                        ports: workload_dto.ports,
                        namespace: namespace.to_string(),
                    };
                    let json = serde_json::to_string(&workload)
                        .map_err(WorkloadServiceError::SerdeError)?;
                    self.etcd_service
                        .put(&new_id, &json)
                        .await
                        .map_err(WorkloadServiceError::EtcdError)?;

                    trace!("Workload created: {:?}", workload);
                    Ok(workload)
                }
                _ => Err(err),
            },
        }
    }

    /// It updates a workload in the etcd
    ///
    /// # Arguments:
    ///
    /// * `workload_dto`: WorkloadDTO
    /// * `workload_id`: The id of the workload to update
    /// * `namespace`: The namespace of the workload
    ///
    /// # Returns:
    ///
    /// A Result<String, WorkloadError>
    pub async fn update_workload(
        &mut self,
        workload_dto: WorkloadDTO,
        workload_name: &str,
        namespace: &str,
    ) -> Result<Workload, WorkloadServiceError> {
        // we get the id before update , and the new id after update
        let new_id = self.id(&workload_dto.name, namespace);
        self.get_workload(workload_name, namespace).await?;
        let workload = Workload {
            id: new_id.to_string(),
            name: workload_dto.name,
            workload_type: Type::Container,
            uri: workload_dto.uri,
            environment: workload_dto.environment.to_vec(),
            resources: Ressources {
                cpu: workload_dto.resources.cpu,
                memory: workload_dto.resources.memory,
                disk: workload_dto.resources.disk,
            },
            ports: workload_dto.ports.to_vec(),
            namespace: namespace.to_string(),
        };
        let json = serde_json::to_string(&workload).map_err(WorkloadServiceError::SerdeError)?;
        self.delete_workload(workload_name, namespace).await;
        self.etcd_service
            .put(&new_id, &json)
            .await
            .map_err(WorkloadServiceError::EtcdError)?;

        trace!("Workload updated: {:?}", workload);
        Ok(workload)
    }

    pub async fn delete_workload(&mut self, workload_name: &str, namespace: &str) {
        let id = self.id(workload_name, namespace);
        _ = self.etcd_service.delete(&id).await;
    }

    pub fn id(&mut self, name: &str, namespace: &str) -> String {
        format!("{}.{}", namespace, name)
    }
}
