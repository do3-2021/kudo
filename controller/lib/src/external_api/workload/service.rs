use super::model::{Workload, WorkloadError, Type, Ressources, WorkloadDTO};
use uuid::Uuid;
use crate::internal_etcd::interface::{EtcdInterface};
use serde_json;

pub struct WorkloadService {
    etcd_service: EtcdInterface,
}

impl WorkloadService {

    pub async fn new() -> Self {
        WorkloadService { 
            etcd_service : EtcdInterface::new().await
        }
    }

    pub async fn get_workload(&mut self,workload_id: &str) -> Result<String, WorkloadError> {
        return match self.etcd_service.get(workload_id).await  {
                Ok(workload) => {
                    Ok(workload)
                },
                Err(_) => Err(WorkloadError::WorkloadNotFound),
        }
    }

    pub async fn get_all_workloads(&mut self) -> Vec<String> {
        match self.etcd_service.get_all().await {
            Ok(workloads) => {
                panic!("{:?}", workloads);
            },
            Err(_) => {
                return vec![];
            }
        }   
    }

    pub async fn create_workload(&mut self, workload_dto : WorkloadDTO ) -> Result<String, WorkloadError> {
        let workload = Workload {
            id: Uuid::new_v4().to_string(),
            name: workload_dto.name,
            workload_type: Type::CONTAINER,
            uri: "http://localhost:8080".to_string(),
            environment: workload_dto.environment.to_vec(),
            resources: Ressources {
                cpu: 0,
                memory: 0,
                disk: 0
            },
            ports: workload_dto.ports.to_vec()
        };
        match self.etcd_service.put(&workload.id, &serde_json::to_string(&workload).unwrap()[..]).await {
            Ok(_) => Ok(serde_json::to_string(&workload).unwrap()),
            Err(e) => Err(WorkloadError::Etcd(e.to_string())),
        }
    }

    pub async fn update_workload(&mut self, workload_id : &str, workload_dto : WorkloadDTO) -> Result<String, WorkloadError> {
        match self.get_workload(workload_id).await  {
            Ok(_) => {
                let workload = Workload {
                    id: workload_id.to_string(),
                    name: workload_dto.name,
                    workload_type: Type::CONTAINER,
                    uri: "http://localhost:8080".to_string(),
                    environment: workload_dto.environment.to_vec(),
                    resources: Ressources {
                        cpu: 0,
                        memory: 0,
                        disk: 0
                    },
                    ports: workload_dto.ports.to_vec()   
                };
                match self.etcd_service.patch(&workload.id, &serde_json::to_string(&workload).unwrap()[..]).await {
                    Ok(_) => Ok(serde_json::to_string(&workload).unwrap()),
                    Err(e) => Err(WorkloadError::Etcd(e.to_string())), 
                }
            },
            Err(_) => Err(WorkloadError::WorkloadNotFound),
        }
    }

    pub async fn delete_workload(&mut self, id: &str) -> Result<(), WorkloadError> {
        match self.get_workload(id).await {
            Ok(_) => {
                match self.etcd_service.delete(id).await {
                    Ok(_) => Ok(()),
                    Err(_) => Err(WorkloadError::WorkloadNotFound),
                }
                
            },
            Err(_) => Err(WorkloadError::WorkloadNotFound),
        }
    }
}
