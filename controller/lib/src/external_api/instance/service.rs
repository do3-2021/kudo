use std::net::{Ipv4Addr, SocketAddrV4};
use std::sync::Arc;

use super::model::{Instance, InstanceError};
use crate::etcd::EtcdClient;
use crate::external_api::workload::model::Workload;
use crate::grpc_client::interface::SchedulerClientInterface;
use proto::controller::InstanceState;
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

    pub async fn generate_ip(&mut self) -> Result<SocketAddrV4, InstanceError> {
        let ip = Ipv4Addr::new(10, 0, 0, 1);
        match self.etcd_service.get("last_ip").await {
            Some(value) => {
                let socket_address: SocketAddrV4 =
                    serde_json::from_str(&value).map_err(InstanceError::SerdeError).unwrap();
                let mut octets = socket_address.ip().octets();
                for i in 0..3 {
                    if octets[3 - i] < 255 {
                        octets[3 - i] += 1;
                        break;
                    } else {
                        octets[3 - i] = 0;
                    }
                }
                let new_ip = SocketAddrV4::new(Ipv4Addr::from(octets), socket_address.port());
                self.etcd_service.put(
                    "last_ip",
                    &serde_json::to_string(&new_ip)
                        .map_err(InstanceError::SerdeError)
                        .unwrap(),
                ).await;
                Ok(new_ip)
            }
            None => {
                let new_ip = SocketAddrV4::new(ip, 0);
                self.etcd_service.put(
                    "last_ip",
                    &serde_json::to_string(&new_ip)
                        .map_err(InstanceError::SerdeError)
                        .unwrap(),
                ).await;
                Ok(new_ip)
            }
        }
    }

    pub async fn retrieve_and_start_instance(
        this: Arc<Mutex<Self>>,
        new_instance: Instance,
    ) -> Result<(), InstanceError> {
        return match this
            .clone()
            .lock()
            .await
            .etcd_service
            .get(new_instance.id.as_str())
            .await
        {
            Some(_) => {
                //Generate an ip for the instance
                let mut instance = new_instance.clone();
                instance.ip = this.clone().lock().await.generate_ip().await?;
                //Spawn a thread to start the instance
                tokio::spawn(async move {
                    loop {
                        let mut stream = this
                            .clone()
                            .lock()
                            .await
                            .grpc_service
                            .create_instance(Request::new(Instance::into(instance.clone())))
                            .await
                            .map_err(|err| InstanceError::Grpc(err.to_string()))
                            .unwrap()
                            .into_inner();

                        let mut last_state = InstanceState::Scheduling;

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

                            last_state = InstanceState::from_i32(instance_status.status)
                                .unwrap_or(InstanceState::Scheduling);
                        }

                        if last_state == InstanceState::Terminated {
                            break;
                        }
                    }
                });

                Ok(())
            }
            None => {
                return Err(InstanceError::InstanceNotFound);
            }
        };
    }

    pub async fn retrieve_and_start_instance_from_workload(
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
                let mut instance = Instance::from(workload_parsed);
                instance.ip = this.clone().lock().await.generate_ip().await?;
                //Spawn a thread to start the instance
                tokio::spawn(async move {
                    loop {
                        let mut stream = this
                            .clone()
                            .lock()
                            .await
                            .grpc_service
                            .create_instance(Request::new(Instance::into(instance.clone())))
                            .await
                            .map_err(|err| InstanceError::Grpc(err.to_string()))
                            .unwrap()
                            .into_inner();

                        let mut last_state = InstanceState::Scheduling;

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

                            last_state = InstanceState::from_i32(instance_status.status)
                                .unwrap_or(InstanceState::Scheduling);
                        }

                        if last_state == InstanceState::Terminated {
                            break;
                        }
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

    pub async fn get_instance(
        &mut self,
        instance_id: &str,
        namespace: &str,
    ) -> Result<Instance, InstanceError> {
        match self.etcd_service.get(instance_id).await {
            Some(instance) => match serde_json::from_str::<Instance>(&instance) {
                Ok(instance) => {
                    if instance.namespace == namespace {
                        Ok(instance)
                    } else {
                        Err(InstanceError::InstanceNotFound)
                    }
                }
                Err(_) => Err(InstanceError::InstanceNotFound),
            },
            None => Err(InstanceError::InstanceNotFound),
        }
    }

    // pub async fn delete_instance(&mut self, workload_id: &str) -> Result<(), InstanceError> {
    // }
}
