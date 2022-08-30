use log::{debug, trace};
use proto::scheduler::instance_service_client::InstanceServiceClient;
use proto::scheduler::{Instance, InstanceIdentifier, InstanceStatus};
use thiserror::Error;
use tonic::transport::{Channel, Error};
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug, Error)]
pub enum SchedulerClientInterfaceError {
    #[error("Error while creating scheduler client: {0}")]
    ConnectionError(Error),
    #[error("Error while sending request to scheduler: {0}")]
    RequestFailed(Status),
}

pub struct SchedulerClientInterface {
    instance_client: InstanceServiceClient<Channel>,
}

impl SchedulerClientInterface {
    pub async fn new(
        instance_client_address: String,
    ) -> Result<Self, SchedulerClientInterfaceError> {
        debug!(
            "Starting gRPC client for scheduler Instance Service on {}",
            instance_client_address,
        );

        let instance_client = InstanceServiceClient::connect(instance_client_address)
            .await
            .map_err(SchedulerClientInterfaceError::ConnectionError)?;

        Ok(Self { instance_client })
    }

    pub async fn create_instance(
        &mut self,
        request: Request<Instance>,
    ) -> Result<Response<Streaming<InstanceStatus>>, SchedulerClientInterfaceError> {
        let response = self
            .instance_client
            .create(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)?;

        trace!("create_instance, response: {:?}", response);
        Ok(response)
    }

    pub async fn destroy_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
        let response = self
            .instance_client
            .destroy(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)?;

        trace!("destroy_instance, response: {:?}", response);
        Ok(response)
    }

    pub async fn start_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
        let response = self
            .instance_client
            .start(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)?;

        trace!("start_instance, response: {:?}", response);
        Ok(response)
    }

    pub async fn stop_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
        let response = self
            .instance_client
            .stop(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)?;

        trace!("stop_instance, response: {:?}", response);
        Ok(response)
    }
}
