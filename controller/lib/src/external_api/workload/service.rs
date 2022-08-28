use std::net::SocketAddr;

use super::filter::FilterService;
use super::model::{Ressources, Type, Workload, WorkloadDTO, WorkloadError};
use crate::etcd::EtcdClient;
use log::debug;
use serde_json;

/// `WorkloadService` is a struct that inpired from Controllers Provider Modules architectures. It can be used as a service in the WorkloadController .A service can use other services.
/// Properties:
///
/// * `etcd_service`: This is the service that will be used to interact with etcd.
/// * `filter_service`: This is the service that will be used to filter the workloads.
pub struct WorkloadService {
    etcd_service: EtcdClient,
    filter_service: FilterService,
}

impl WorkloadService {
    pub async fn new(etcd_address: &SocketAddr) -> Result<WorkloadService, WorkloadError> {
        let inner = WorkloadService {
            etcd_service: EtcdClient::new(etcd_address.to_string())
                .await
                .map_err(|err| WorkloadError::Etcd(err.to_string()))?,
            filter_service: FilterService::new(),
        };
        Ok(inner)
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
    pub async fn get_workload(
        &mut self,
        workload_name: &str,
        namespace: &str,
    ) -> Result<Workload, WorkloadError> {
        let id = self.get_id(workload_name, namespace);
        match self.etcd_service.get(&id).await {
            Some(workload) => {
                let workload: Workload = serde_json::from_str(&workload)
                    .map_err(|err| WorkloadError::JsonToWorkload(err.to_string()))?;
                if workload.namespace == namespace {
                    Ok(workload)
                } else {
                    Err(WorkloadError::WorkloadNotFound)
                }
            }
            None => Err(WorkloadError::WorkloadNotFound),
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
    ) -> Vec<Workload> {
        let mut new_vec: Vec<Workload> = Vec::new();
        match self.etcd_service.get_all().await {
            Some(workloads) => {
                debug!("{:?}", workloads);
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
                        Err(_) => return vec![],
                    }
                }
                if limit > 0 {
                    new_vec = self.filter_service.limit(&new_vec, limit);
                }
                new_vec
            }
            None => {
                debug!("No workloads found");
                vec![]
            }
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
    ) -> Result<Workload, WorkloadError> {
        let new_id = self.get_id(&workload_dto.name, namespace);
        match self.get_workload(&workload_dto.name, namespace).await {
            Ok(workload) => Err(WorkloadError::NameAlreadyExists(workload.name)),
            Err(err) => match err {
                WorkloadError::WorkloadNotFound => {
                    let workload = Workload {
                        id: new_id.to_string(),
                        name: workload_dto.name,
                        workload_type: Type::Container,
                        uri: workload_dto.uri,
                        environment: workload_dto.environment,
                        resources: Ressources {
                            cpu: 0,
                            memory: 0,
                            disk: 0,
                        },
                        ports: workload_dto.ports,
                        namespace: namespace.to_string(),
                    };
                    let json = serde_json::to_string(&workload)
                        .map_err(|err| WorkloadError::WorkloadToJson(err.to_string()))?;
                    self.etcd_service
                        .put(&new_id, &json)
                        .await
                        .map_err(|err| WorkloadError::Etcd(err.to_string()))?;
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
    ) -> Result<Workload, WorkloadError> {
        // we get the id before update , and the new id after update
        let old_id = self.get_id(workload_name, namespace);
        let new_id = self.get_id(&workload_dto.name, namespace);
        self.get_workload(&old_id, namespace).await?;
        let workload = Workload {
            id: new_id.to_string(),
            name: workload_dto.name,
            workload_type: Type::Container,
            uri: workload_dto.uri,
            environment: workload_dto.environment.to_vec(),
            resources: Ressources {
                cpu: 0,
                memory: 0,
                disk: 0,
            },
            ports: workload_dto.ports.to_vec(),
            namespace: namespace.to_string(),
        };
        let json = serde_json::to_string(&workload)
            .map_err(|err| WorkloadError::WorkloadToJson(err.to_string()))?;
        self.etcd_service
            .put(&new_id, &json)
            .await
            .map_err(|err| WorkloadError::Etcd(err.to_string()))?;
        Ok(workload)
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
    pub async fn delete_workload(&mut self, workload_name: &str, namespace: &str) {
        let id = self.get_id(workload_name, namespace);
        _ = self.etcd_service.delete(&id).await;
    }

    /// Workload id is a string that is the concatenation of the namespace and name
    ///
    /// Arguments:
    ///
    /// * `name`: The name of the service
    /// * `namespace`: The namespace of the service.
    ///
    /// Returns:
    ///
    /// A string
    pub fn get_id(&mut self, name: &str, namespace: &str) -> String {
        format!("{}.{}", namespace, name)
    }
}
