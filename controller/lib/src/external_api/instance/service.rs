use std::net::{Ipv4Addr, SocketAddr};
use std::sync::Arc;

use super::filter::InstanceFilterService;
use super::model::Instance;
use crate::etcd::{EtcdClient, EtcdClientError};
use crate::external_api::workload::model::Workload;
use crate::grpc_client::interface::{SchedulerClientInterface, SchedulerClientInterfaceError};
use log::{debug, trace};
use proto::controller::InstanceState;
use serde_json;
use thiserror::Error;
use tokio::sync::Mutex;
use tonic::{Request, Status};

#[derive(Debug, Error)]
pub enum InstanceServiceError {
    #[error("Etcd client error: {0}")]
    EtcdError(EtcdClientError),
    #[error("Serde error: {0}")]
    SerdeError(serde_json::Error),
    #[error("Scheduler client error: {0}")]
    SchedulerClientInterfaceError(SchedulerClientInterfaceError),
    #[error("Workload {0} not found")]
    WorkloadNotFound(String),
    #[error("Instance {0} not found")]
    InstanceNotFound(String),
    #[error("Stream error: {0}")]
    StreamError(Status),
}

pub struct InstanceService {
    grpc_service: SchedulerClientInterface,
    etcd_service: EtcdClient,
    filter_service: InstanceFilterService,
}

// `InstanceService` is a struct that is inspired from Controllers Provider Modules architectures. It is used as a service in the InstanceController. A service can use other services.
/// Properties:
///
/// * `grpc_service`: This is the service that will be used to filter to interact with grpc.
/// * `etcd_service`: This is the service that will be used to interact with etcd.
/// * `filter_service`: This is the service that will be used to filter the instances.

impl InstanceService {
    pub async fn new(
        grpc_address: &str,
        etcd_address: &SocketAddr,
    ) -> Result<Self, InstanceServiceError> {
        Ok(InstanceService {
            grpc_service: SchedulerClientInterface::new(grpc_address.to_string())
                .await
                .map_err(InstanceServiceError::SchedulerClientInterfaceError)?,
            etcd_service: EtcdClient::new(etcd_address.to_string())
                .await
                .map_err(InstanceServiceError::EtcdError)?,
            filter_service: InstanceFilterService::new(),
        })
    }

    /// `generate_ip` is an async function that generate an ip address for a instance.
    /// # Description:
    /// It stores the last ip used in ETCD and increment it by 1 every time it is used .
    /// * Get all workload in the namespace
    /// # Return
    /// An Result<Ipv4Addr, InstanceServiceError>
    pub async fn generate_ip(&mut self) -> Result<Ipv4Addr, InstanceServiceError> {
        let mut ip = Ipv4Addr::new(10, 0, 0, 1);
        match self.etcd_service.get("last_ip").await {
            Some(value) => {
                let ip_address: Ipv4Addr =
                    serde_json::from_str(&value).map_err(InstanceServiceError::SerdeError)?;
                let mut octets = ip_address.octets();
                for i in 0..3 {
                    if octets[3 - i] < 255 {
                        octets[3 - i] += 1;
                        break;
                    } else {
                        octets[3 - i] = 0;
                    }
                }
                ip = Ipv4Addr::from(octets);
                self.etcd_service
                    .put(
                        "last_ip",
                        &serde_json::to_string(&ip).map_err(InstanceServiceError::SerdeError)?,
                    )
                    .await
                    .map_err(InstanceServiceError::EtcdError)?;
                Ok(ip)
            }
            None => {
                self.etcd_service
                    .put(
                        "last_ip",
                        &serde_json::to_string(&ip).map_err(InstanceServiceError::SerdeError)?,
                    )
                    .await
                    .map_err(InstanceServiceError::EtcdError)?;
                Ok(ip)
            }
        }
    }

    /// It creates a new instance in etcd and call the `schedule` function of the grpc service
    ///
    /// # Arguments:
    ///
    /// * `this`: the method is called on the `this` object. Used to handle threads.
    /// * `workload_id`: The id of the workload that the instance will be created for.
    ///
    /// # Returns:
    ///
    /// A Result.
    pub async fn retrieve_and_start_instance_from_workload(
        this: Arc<Mutex<Self>>,
        workload_id: &str,
    ) -> Result<(), InstanceServiceError> {
        let ip = this.clone().lock().await.generate_ip().await?;
        match this
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
                instance.ip = ip;
                Self::schedule_instance(this, instance);

                trace!("Instance creating from workload {}", workload);
                Ok(())
            }
            None => Err(InstanceServiceError::WorkloadNotFound(
                workload_id.to_string(),
            )),
        }
    }

    /// Schedule an new instance by calling the GRPC client and recursively call this function if the instance is not scheduled.
    ///
    /// # Arguments:
    ///
    /// * `this`: the method is called on the `this` object. Used to handle threads.
    /// * `instance`: The instance to schedule
    pub fn schedule_instance(this: Arc<Mutex<Self>>, mut instance: Instance) {
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
                    .map_err(InstanceServiceError::SchedulerClientInterfaceError)
                    .unwrap()
                    .into_inner();

                let mut last_state = InstanceState::Scheduling;

                while let Some(instance_status) = stream
                    .message()
                    .await
                    .map_err(InstanceServiceError::StreamError)
                    .unwrap()
                {
                    instance.update_instance(instance_status.clone());

                    this.clone()
                        .lock()
                        .await
                        .etcd_service
                        .put(
                            &instance.id,
                            &serde_json::to_string(&instance)
                                .map_err(InstanceServiceError::SerdeError)
                                .unwrap(),
                        )
                        .await
                        .map_err(InstanceServiceError::EtcdError)
                        .unwrap();

                    trace!("Instance status received : {:?}", instance);

                    last_state = InstanceState::from_i32(instance_status.status)
                        .unwrap_or(InstanceState::Scheduling);
                }

                if last_state == InstanceState::Terminated {
                    break;
                }

                instance.num_restarts += 1;

                debug!("Restarting instance {}", instance.id);

                this.clone()
                    .lock()
                    .await
                    .etcd_service
                    .put(
                        &instance.id,
                        &serde_json::to_string(&instance)
                            .map_err(InstanceServiceError::SerdeError)
                            .unwrap(),
                    )
                    .await
                    .map_err(InstanceServiceError::EtcdError)
                    .unwrap();
            }
        });
    }

    /// Delete an instance from etcd and call the `destroy_instance` function of the grpc service
    ///
    /// # Arguments:
    ///
    /// * `instance`: The instance to delete
    ///
    /// # Returns:
    ///
    /// A Result.
    pub async fn delete_instance(
        &mut self,
        instance: Instance,
    ) -> Result<(), InstanceServiceError> {
        self.etcd_service
            .delete(&instance.id)
            .await
            .ok_or_else(|| InstanceServiceError::InstanceNotFound(instance.clone().id))?;
        self.grpc_service
            .destroy_instance(Request::new(proto::scheduler::InstanceIdentifier {
                id: instance.clone().id,
            }))
            .await
            .map_err(InstanceServiceError::SchedulerClientInterfaceError)?;

        trace!("Instance {:?} deleted", instance);
        Ok(())
    }

    /// Get an instance by it's name
    ///
    /// # Arguments:
    ///
    /// * `namespace`: The namespace of the instance
    ///
    /// # Returns:
    ///
    /// A Instance.
    pub async fn get_instance(
        &mut self,
        instance_name: &str,
    ) -> Result<Instance, InstanceServiceError> {
        match self
            .etcd_service
            .get(format!("instance.{}", instance_name).as_str())
            .await
        {
            Some(instance_str) => {
                let instance = serde_json::from_str::<Instance>(&instance_str)
                    .map_err(InstanceServiceError::SerdeError)?;

                trace!("Instance found: {:?}", instance);
                Ok(instance)
            }
            None => Err(InstanceServiceError::InstanceNotFound(
                instance_name.to_string(),
            )),
        }
    }

    /// This function gets all instances from etcd, filters them by namespace and slice the result by limit and offset
    /// If there is an error , the function always return an empty vector
    /// # Arguments:
    ///
    /// * `limit`: The number of instances to return.
    /// * `offset`: The offset of the instances to be returned.
    /// * `namespace`: The namespace to filter by.
    ///
    /// # Returns:
    ///
    /// A vector of instances
    pub async fn get_instances(
        &mut self,
        limit: u32,
        offset: u32,
        namespace: &str,
    ) -> Vec<Instance> {
        let mut vec: Vec<Instance> = Vec::new();
        match self.etcd_service.get_all().await {
            Some(instances) => {
                for instance in instances {
                    if let Ok(instance) = serde_json::from_str::<Instance>(&instance) {
                        if instance.namespace == namespace {
                            vec.push(instance);
                        }
                    }
                }
                if offset > 0 {
                    match self.filter_service.offset(&vec, limit) {
                        Ok(instances) => vec = instances,
                        Err(_) => return vec![],
                    }
                }
                if limit > 0 {
                    vec = self.filter_service.limit(&vec, limit);
                }
            }
            None => return vec![],
        }

        trace!("Instances found : {:?}", vec);
        vec
    }
}
