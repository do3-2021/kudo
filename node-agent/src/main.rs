use tonic::{transport::Server, Request, Response, Status};

pub mod node_agent {
    tonic::include_proto!("node_agent");
}

use node_agent::instance_service_server::{InstanceService, InstanceServiceServer};
use node_agent::{Instance, InstanceStatus, SignalInstruction, Status as WorkloadStatus};

#[derive(Debug, Default)]
pub struct InstanceServiceController {}

#[tonic::async_trait]
impl InstanceService for InstanceServiceController {
    type createStream = tonic::Streaming<InstanceStatus>;

    async fn create(
        &self,
        _request: Request<Instance>,
    ) -> Result<Response<Self::createStream>, Status> {
        // todo
        println!("create methode");

        Ok(Response::new(Self::createStream::new(
            "uuid1",
            WorkloadStatus::Starting,
            "description",
        )))
    }

    async fn signal(&self, _request: Request<SignalInstruction>) -> Result<Response<()>, Status> {
        // TODO
        let message = _request.into_inner();
        println!("{:?}", message);

        Ok(Response::new(()))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:56789".parse()?;
    let instance_service_controller = InstanceServiceController::default();

    println!("Node Agent server listening on {}", addr);

    Server::builder()
        .add_service(InstanceServiceServer::new(instance_service_controller))
        .serve(addr)
        .await?;

    Ok(())
}
