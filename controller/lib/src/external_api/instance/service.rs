use std::sync::Arc;

use super::model::{Instance, InstanceError};
use crate::external_api::workload::model::Workload;
use crate::grpc_client::interface::SchedulerClientInterface;
use crate::etcd::EtcdClient;
use serde_json;
use tokio::sync::Mutex;
use tonic::Request;

pub struct InstanceService {
    grpc_service: SchedulerClientInterface,
    etcd_service: EtcdClient,
}

impl InstanceService {
    pub async fn new(grpc_address: &str) -> Self {
        InstanceService {
            grpc_service: SchedulerClientInterface::new(grpc_address.to_string())
                .await
                .unwrap(),
            etcd_service: EtcdClient::new("".to_string()).await.unwrap(),
        }
    }
    // pub async fn generate_ip_adress() -> std::net::Ipv4Addr {
    //     let mut ip = std::net::Ipv4Addr::new(10,0,0,0)
    //     match etcd_service.get("last_ip").await {
    //         Ok(value) => 
    //         Err(_) => ip
    //     }
    //     etcd_service.put("last_ip", ip.to_string())
    // }

    pub async fn retrieve_and_start_instance(
        this: Arc<Mutex<Self>>,
        workload_id: &str,
    ) -> Result<(), InstanceError> {
        return match this
            .clone()
            .lock()
            .await
            .etcd_service
            .get(workload_id)
            .await
        {
            Some(workload) => {
                let workload_parsed: Workload = serde_json::from_str(&workload).unwrap();
                let instance = Instance::from(workload_parsed);
                //TODO Start instance
                //Spawn a thread to start the instance
                let mut stream = this
                    .clone()
                    .lock()
                    .await
                    .grpc_service
                    .create_instance(Request::new(Instance::into(instance.clone())))
                    .await
                    .map_err(|err| InstanceError::Grpc(err.to_string()))?
                    .into_inner();

                tokio::spawn(async move {
                    while let Some(instance_status) = stream
                        .message()
                        .await
                        .map_err(|err| InstanceError::Grpc(err.to_string()))
                        .unwrap()
                    {
                        this.clone()
                            .lock()
                            .await
                            .etcd_service
                            .put(
                                &instance_status.id,
                                &serde_json::to_string(&instance_status)
                                    .map_err(InstanceError::SerdeError)
                                    .unwrap(),
                            )
                            .await
                            .map_err(|err| InstanceError::Etcd(err.to_string()))
                            .unwrap();

                        // if proto::controller::InstanceState::from_i32(instance_status.status)
                        //     .unwrap_or(InstanceState::Scheduling)
                        //     == InstanceState::Failed
                        // {
                        //     super::service::InstanceService::retrieve_and_start_instance(
                        //         this,
                        //         instance.id.as_str(),
                        //     )
                        //     .await
                        //     .unwrap();
                        //     break;
                        // }
                    }
                });

                Ok(())
            }
            None => Err(InstanceError::InstanceNotFound),
        };
    }

    pub async fn delete_instance(&mut self, instance: Instance) -> Result<(), InstanceError> {
        match self.etcd_service.delete(instance.id.as_str()).await {
            Some(_) => {
                match self
                    .grpc_service
                    .destroy_instance(Request::new(proto::scheduler::InstanceIdentifier {
                        id: instance.id,
                    }))
                    .await
                {
                    Ok(_) => Ok(()),
                    Err(_) => Err(InstanceError::Grpc("Error stopping instance".to_string())),
                }
            }
            None => Err(InstanceError::InstanceNotFound),
        }
    }

    pub async fn get_instance(&mut self, instance_id: &str) -> Result<Instance, InstanceError> {
        match self.etcd_service.get(instance_id).await {
            Some(instance) => match serde_json::from_str::<Instance>(&instance) {
                Ok(instance) => Ok(instance),
                Err(_) => Err(InstanceError::InstanceNotFound),
            },
            None => Err(InstanceError::InstanceNotFound),
        }
    }

    // pub async fn delete_instance(&mut self, workload_id: &str) -> Result<(), InstanceError> {
    // }
}
