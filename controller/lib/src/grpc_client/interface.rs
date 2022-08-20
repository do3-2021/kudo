use log::{error, info};
use proto::scheduler::instance_service_client::InstanceServiceClient;
use proto::scheduler::{Instance, InstanceIdentifier, InstanceStatus};
use tonic::transport::{Channel, Error};
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug)]
pub enum SchedulerClientInterfaceError {
    ConnectionError(Error),
    RequestFailed(Status),
}

pub struct SchedulerClientInterface {
    instance_client: InstanceServiceClient<Channel>,
}

impl SchedulerClientInterface {
    pub async fn new(
        instance_client_address: String,
    ) -> Result<Self, SchedulerClientInterfaceError> {
        info!(
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
        let remote_address = match request.remote_addr() {
            Some(addr) => addr.to_string(),
            None => {
                error!("\"create_instance\" Failed to get remote address from request");
                "Error getting address".to_string()
            }
        };

        info!(
            "Calling gRPC procedure \"create_instance\" to {}",
            remote_address
        );

        self.instance_client
            .create(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)
    }

    pub async fn destroy_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
        let remote_address = match request.remote_addr() {
            Some(addr) => addr.to_string(),
            None => {
                error!("\"create_instance\" Failed to get remote address from request");
                "Error getting address".to_string()
            }
        };

        info!(
            "Calling gRPC procedure \"destroy_instance\" to {}",
            remote_address
        );

        self.instance_client
            .destroy(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)
    }

    pub async fn start_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
        let remote_address = match request.remote_addr() {
            Some(addr) => addr.to_string(),
            None => {
                error!("\"create_instance\" Failed to get remote address from request");
                "Error getting address".to_string()
            }
        };

        info!(
            "Calling gRPC procedure \"start_instance\" to {}",
            remote_address
        );

        self.instance_client
            .start(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)
    }

    pub async fn stop_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
        let remote_address = match request.remote_addr() {
            Some(addr) => addr.to_string(),
            None => {
                error!("\"create_instance\" Failed to get remote address from request");
                "Error getting address".to_string()
            }
        };

        info!(
            "Calling gRPC procedure \"stop_instance\" to {}",
            remote_address
        );

        self.instance_client
            .stop(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)
    }
}
