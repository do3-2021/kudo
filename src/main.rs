use tonic::{transport::Server, Request, Response, Status};

use network::{CreateInterfaceRequest, CreateInterfaceResponse, CreateContainerNetworkInterfaceRequest, CreateContainerNetworkInterfaceResponse, DeleteInterfaceRequest, Empty};
use network::network_server::{Network, NetworkServer};

pub mod network {
    tonic::include_proto!("network");
}

#[derive(Debug, Default)]
pub struct NetworkHandler {}

#[tonic::async_trait]
impl Network for NetworkHandler {
    async fn create_interface(
        &self,
        request: Request<CreateInterfaceRequest>,
    ) -> Result<Response<CreateInterfaceResponse>, Status> {

        let reply = CreateInterfaceResponse {
            interface_name: format!("Hello !").into(),
        };

        println!("create_interface; {:?}", reply.interface_name);
        request.into_inner().ports.iter().for_each(|port| {
            println!("create_interface; port: {:?}", port);
        });

        Ok(Response::new(reply))
    }

    async fn delete_interface(
        &self, 
        request: Request<DeleteInterfaceRequest>,
    ) -> Result<Response<Empty>, Status> {
        println!("delete_interface: {:?}", request.into_inner().interface_name);
        Ok(Response::new(Empty {}))
    }

    async fn create_container_network_interface(
        &self,
        request: Request<CreateContainerNetworkInterfaceRequest>,
    ) -> Result<Response<CreateContainerNetworkInterfaceResponse>, Status> {
        let inner = request.into_inner();
        println!("create_container_network_interface: {}, {}", inner.ip_address, inner.sub_network);
        Ok(Response::new(CreateContainerNetworkInterfaceResponse {
            interface_name: format!("Hello !").into(),
        }))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let greeter = NetworkHandler::default();

    Server::builder()
        .add_service(NetworkServer::new(greeter))
        .serve(addr)
        .await?;

    Ok(())
}