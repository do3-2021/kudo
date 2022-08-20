<<<<<<< HEAD
use log::{error, info};
use proto::scheduler::instance_service_client::InstanceServiceClient;
use proto::scheduler::{Instance, InstanceIdentifier, InstanceStatus};
use tonic::transport::{Channel, Error};
use tonic::{Request, Response, Status, Streaming};

#[derive(Debug)]
=======
use log::info;
use proto::scheduler::instance_service_client::InstanceServiceClient;
use proto::scheduler::node_service_client::NodeServiceClient;
use proto::scheduler::{
    Instance, InstanceIdentifier, InstanceStatus, NodeRegisterRequest, NodeStatus,
    NodeUnregisterRequest, NodeUnregisterResponse,
};
use tonic::transport::{Channel, Error};
use tonic::{Request, Response, Status, Streaming};

>>>>>>> feat: implement grpc client
pub enum SchedulerClientInterfaceError {
    ConnectionError(Error),
    RequestFailed(Status),
}

pub struct SchedulerClientInterface {
<<<<<<< HEAD
=======
    node_client: NodeServiceClient<Channel>,
>>>>>>> feat: implement grpc client
    instance_client: InstanceServiceClient<Channel>,
}

impl SchedulerClientInterface {
    pub async fn new(
<<<<<<< HEAD
        instance_client_address: String,
    ) -> Result<Self, SchedulerClientInterfaceError> {
        info!(
=======
        node_client_address: String,
        instance_client_address: String,
    ) -> Result<Self, SchedulerClientInterfaceError> {
        info!(
            "Starting gRPC client for scheduler Node Service on {}",
            node_client_address,
        );

        let node_client = match NodeServiceClient::connect(node_client_address).await {
            Ok(client) => client,
            Err(e) => return Err(SchedulerClientInterfaceError::ConnectionError(e)),
        };

        info!(
>>>>>>> feat: implement grpc client
            "Starting gRPC client for scheduler Instance Service on {}",
            instance_client_address,
        );

<<<<<<< HEAD
        let instance_client = InstanceServiceClient::connect(instance_client_address)
            .await
            .map_err(SchedulerClientInterfaceError::ConnectionError)?;

        Ok(Self { instance_client })
=======
        let instance_client = match InstanceServiceClient::connect(instance_client_address).await {
            Ok(client) => client,
            Err(e) => return Err(SchedulerClientInterfaceError::ConnectionError(e)),
        };

        Ok(Self {
            node_client,
            instance_client,
        })
    }

    pub async fn register_node(
        &mut self,
        request: Request<NodeRegisterRequest>,
    ) -> Result<Response<Streaming<NodeStatus>>, SchedulerClientInterfaceError> {
        let remote_address = request.remote_addr().unwrap();

        info!(
            "Calling gRPC procedure \"register_node\" to {}",
            remote_address
        );

        match self.node_client.register(request).await {
            Ok(response) => Ok(response),
            Err(e) => Err(SchedulerClientInterfaceError::RequestFailed(e)),
        }
    }

    pub async fn unregister_node(
        &mut self,
        request: Request<NodeUnregisterRequest>,
    ) -> Result<Response<NodeUnregisterResponse>, SchedulerClientInterfaceError> {
        let remote_address = request.remote_addr().unwrap();

        info!(
            "Calling gRPC procedure \"unregister_node\" to {}",
            remote_address
        );

        match self.node_client.unregister(request).await {
            Ok(response) => Ok(response),
            Err(e) => Err(SchedulerClientInterfaceError::RequestFailed(e)),
        }
>>>>>>> feat: implement grpc client
    }

    pub async fn create_instance(
        &mut self,
        request: Request<Instance>,
    ) -> Result<Response<Streaming<InstanceStatus>>, SchedulerClientInterfaceError> {
<<<<<<< HEAD
        let remote_address = match request.remote_addr() {
            Some(addr) => addr.to_string(),
            None => {
                error!("\"create_instance\" Failed to get remote address from request");
                "Error getting address".to_string()
            }
        };
=======
        let remote_address = request.remote_addr().unwrap();
>>>>>>> feat: implement grpc client

        info!(
            "Calling gRPC procedure \"create_instance\" to {}",
            remote_address
        );

<<<<<<< HEAD
        self.instance_client
            .create(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)
=======
        match self.instance_client.create(request).await {
            Ok(response) => Ok(response),
            Err(e) => Err(SchedulerClientInterfaceError::RequestFailed(e)),
        }
>>>>>>> feat: implement grpc client
    }

    pub async fn destroy_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
<<<<<<< HEAD
        let remote_address = match request.remote_addr() {
            Some(addr) => addr.to_string(),
            None => {
                error!("\"create_instance\" Failed to get remote address from request");
                "Error getting address".to_string()
            }
        };
=======
        let remote_address = request.remote_addr().unwrap();
>>>>>>> feat: implement grpc client

        info!(
            "Calling gRPC procedure \"destroy_instance\" to {}",
            remote_address
        );

<<<<<<< HEAD
        self.instance_client
            .destroy(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)
=======
        match self.instance_client.destroy(request).await {
            Ok(response) => Ok(response),
            Err(e) => Err(SchedulerClientInterfaceError::RequestFailed(e)),
        }
>>>>>>> feat: implement grpc client
    }

    pub async fn start_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
<<<<<<< HEAD
        let remote_address = match request.remote_addr() {
            Some(addr) => addr.to_string(),
            None => {
                error!("\"create_instance\" Failed to get remote address from request");
                "Error getting address".to_string()
            }
        };
=======
        let remote_address = request.remote_addr().unwrap();
>>>>>>> feat: implement grpc client

        info!(
            "Calling gRPC procedure \"start_instance\" to {}",
            remote_address
        );

<<<<<<< HEAD
        self.instance_client
            .start(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)
=======
        match self.instance_client.start(request).await {
            Ok(response) => Ok(response),
            Err(e) => Err(SchedulerClientInterfaceError::RequestFailed(e)),
        }
>>>>>>> feat: implement grpc client
    }

    pub async fn stop_instance(
        &mut self,
        request: Request<InstanceIdentifier>,
    ) -> Result<Response<()>, SchedulerClientInterfaceError> {
<<<<<<< HEAD
        let remote_address = match request.remote_addr() {
            Some(addr) => addr.to_string(),
            None => {
                error!("\"create_instance\" Failed to get remote address from request");
                "Error getting address".to_string()
            }
        };
=======
        let remote_address = request.remote_addr().unwrap();
>>>>>>> feat: implement grpc client

        info!(
            "Calling gRPC procedure \"stop_instance\" to {}",
            remote_address
        );

<<<<<<< HEAD
        self.instance_client
            .stop(request)
            .await
            .map_err(SchedulerClientInterfaceError::RequestFailed)
=======
        match self.instance_client.stop(request).await {
            Ok(response) => Ok(response),
            Err(e) => Err(SchedulerClientInterfaceError::RequestFailed(e)),
        }
>>>>>>> feat: implement grpc client
    }
}
