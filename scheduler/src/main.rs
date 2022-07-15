use std::pin::Pin;

use futures_core::Stream;
use futures_util::StreamExt;
use proto::scheduler::agent_service_server::{AgentService, AgentServiceServer};
use proto::scheduler::controller_service_server::{ControllerService, ControllerServiceServer};
use proto::scheduler::{
    Instance, InstanceAction, InstanceStatus, NodeRegisterRequest, NodeRegisterResponse, NodeStatus,
};
use tokio::sync::mpsc::{channel, Receiver, Sender};
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
struct Storage {
    instances: Vec<Instance>,
}

impl Storage {
    fn new() -> Self {
        Storage {
            instances: Vec::new(),
        }
    }

    fn add_instance(&mut self, instance: Instance) {
        self.instances.push(instance);
    }

    fn update_instance(&mut self, id: &str, status: i32) {
        let mut old_instance = self.get_instance_mut(id).unwrap();
        old_instance.status = status;
    }

    fn get_instance(&self, id: &str) -> Option<&Instance> {
        self.instances.iter().find(|i| i.id == id)
    }

    fn get_instance_mut(&mut self, id: &str) -> Option<&mut Instance> {
        self.instances.iter_mut().find(|i| i.id == id)
    }

    fn get_instances(&self) -> &Vec<Instance> {
        &self.instances
    }
}

#[derive(Debug, Default)]
struct MyAgentService {}

#[tonic::async_trait]
impl AgentService for MyAgentService {
    async fn node_register(
        &self,
        request: Request<NodeRegisterRequest>,
    ) -> Result<Response<NodeRegisterResponse>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        Err(Status::cancelled("")) // Send back our formatted greeting
    }

    async fn node_status_update(
        &self,
        request: Request<tonic::Streaming<NodeStatus>>,
    ) -> Result<Response<()>, Status> {
        // Return an instance of type HelloReply
        let mut stream = request.into_inner();

        while let Some(status) = stream.next().await {
            println!("Got a status: {:?}", status);
        }

        Ok(Response::new(()))
    }

    async fn instance_status_update(
        &self,
        request: Request<tonic::Streaming<InstanceStatus>>,
    ) -> Result<Response<()>, Status> {
        // Return an instance of type HelloReply
        let mut stream = request.into_inner();

        while let Some(status) = stream.next().await {
            println!("Got a status: {:?}", status);
        }

        Ok(Response::new(()))
    }
}

#[derive(Debug)]
struct MyControllerService {
    sender: Sender<Event>,
}

impl MyControllerService {
    fn new(sender: Sender<Event>) -> Self {
        MyControllerService { sender }
    }
}

#[tonic::async_trait]
impl ControllerService for MyControllerService {
    async fn instance_create(&self, request: Request<Instance>) -> Result<Response<()>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let event: Event = Event::InstanceCreate(request.into_inner());
        self.sender.send(event).await.ok();

        Ok(Response::new(())) // Send back our formatted greeting
    }

    async fn instance_update(
        &self,
        request: Request<InstanceAction>,
    ) -> Result<Response<()>, Status> {
        // Return an instance of type HelloReply
        println!("Got a request: {:?}", request);

        let event: Event = Event::InstanceUpdate(request.into_inner());
        self.sender.send(event).await.ok();

        Ok(Response::new(())) // Send back our formatted greeting
    }
}

struct Manager {
    storage: Storage,
    receiver: Receiver<Event>,
}

impl Manager {
    fn new(receiver: Receiver<Event>) -> Self {
        Manager {
            storage: Storage::new(),
            receiver,
        }
    }

    async fn run(&mut self) {
        while let Some(event) = self.receiver.recv().await {
            match event {
                Event::InstanceCreate(instance) => {
                    self.storage.add_instance(instance);
                }
                Event::InstanceUpdate(instance_action) => {
                    self.storage.update_instance(&instance_action.id, 3);
                }
            }

            println!("{:?}", self.storage.get_instances());
        }
    }
}

enum Event {
    InstanceCreate(Instance),
    InstanceUpdate(InstanceAction),
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "0.0.0.0:50051".parse()?;
    let (sender, receiver) = channel::<Event>(4096);
    let controllerService = MyControllerService::new(sender);
    let agentService = MyAgentService::default();

    let mut manager = Manager::new(receiver);
    tokio::spawn(async move {
        manager.run().await;
    });

    Server::builder()
        .add_service(ControllerServiceServer::new(controllerService))
        .add_service(AgentServiceServer::new(agentService))
        .serve(addr)
        .await?;

    Ok(())
}
