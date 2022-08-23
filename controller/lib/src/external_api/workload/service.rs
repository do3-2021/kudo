use super::filter::FilterService;
use super::model::{Ressources, Type, Workload, WorkloadDTO, WorkloadError};
use crate::etcd::EtcdClient;
use serde_json;
use uuid::Uuid;

pub struct WorkloadService {
    etcd_service: EtcdClient,
    filter_service: FilterService,
}

impl WorkloadService {
    pub async fn new() -> Self {
        WorkloadService {
            etcd_service: EtcdClient::new("".to_string()).await.unwrap(),
            filter_service: FilterService::new(),
        }
    }

    pub async fn get_workload(
        &mut self,
        workload_id: &str,
        namespace: &str,
    ) -> Result<String, WorkloadError> {
        return match self.etcd_service.get(workload_id).await {
            Some(workload) => {
                let workload: Workload = serde_json::from_str(&workload).unwrap();
                if workload.namespace == namespace {
                    Ok(serde_json::to_string(&workload).unwrap())
                } else {
                    Err(WorkloadError::WorkloadNotFound)
                }
            }
            None => Err(WorkloadError::WorkloadNotFound),
        };
    }

    pub async fn get_all_workloads(
        &mut self,
        limit: u32,
        offset: u32,
        namespace: &str,
    ) -> Vec<Workload> {
        let mut new_vec: Vec<Workload> = Vec::new();
        match self.etcd_service.get_all().await {
            Ok(workloads) => {
                for workload in workloads {
                    new_vec.push(serde_json::from_str(&workload).unwrap());
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
                new_vec = self.filter_service.filter_by_namespace(&new_vec, namespace);
                new_vec
            }
            Err(_) => {
                return vec![];
            }
        }
    }

    pub async fn create_workload(
        &mut self,
        workload_dto: WorkloadDTO,
        namespace: &str,
    ) -> Result<String, WorkloadError> {
        let workload = Workload {
            id: Uuid::new_v4().to_string(),
            name: workload_dto.name,
            workload_type: Type::CONTAINER,
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
        match self
            .etcd_service
            .put(&workload.id, &serde_json::to_string(&workload).unwrap())
            .await
        {
            Ok(_) => Ok(serde_json::to_string(&workload).unwrap()),
            Err(e) => Err(WorkloadError::Etcd(e.to_string())),
        }
    }

    pub async fn update_workload(
        &mut self,
        workload_dto: WorkloadDTO,
        workload_id: &str,
        namespace: &str,
    ) -> Result<String, WorkloadError> {
        match self.get_workload(workload_id, namespace).await {
            Ok(_) => {
                let workload = Workload {
                    id: workload_id.to_string(),
                    name: workload_dto.name,
                    workload_type: Type::CONTAINER,
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
                match self
                    .etcd_service
                    .put(&workload.id, &serde_json::to_string(&workload).unwrap()[..])
                    .await
                {
                    Ok(_) => Ok(serde_json::to_string(&workload).unwrap()),
                    Err(e) => Err(WorkloadError::Etcd(e.to_string())),
                }
            }
            Err(_) => Err(WorkloadError::WorkloadNotFound),
        }
    }

    pub async fn delete_workload(
        &mut self,
        id: &str,
        namespace: &str,
    ) -> Result<(), WorkloadError> {
        match self.get_workload(id, namespace).await {
            Ok(_) => match self.etcd_service.delete(id).await {
                Some(_) => Ok(()),
                None => Err(WorkloadError::WorkloadNotFound),
            },
            Err(_) => Err(WorkloadError::WorkloadNotFound),
        }
    }
}
