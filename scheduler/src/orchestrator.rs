use std::{net::IpAddr, sync::Arc};

use log;
use proto::{
    controller::node_service_client::NodeServiceClient,
    scheduler::{Instance, InstanceStatus, NodeStatus, Resource, ResourceSummary, Status},
};
use tokio::sync::{
    mpsc::{self},
    Mutex,
};
use tonic::Streaming;

use crate::{
    config::Config,
    instance::scheduled::InstanceScheduled,
    node::{registered::NodeRegistered, Node},
    storage::{IStorage, Storage},
    InstanceIdentifier, NodeIdentifier, OrchestratorError,
};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Orchestrator {
    nodes: Storage<NodeRegistered>,
    config: Arc<Config>,
}

impl Orchestrator {
    pub fn new(nodes: Storage<NodeRegistered>, config: Arc<Config>) -> Self {
        Orchestrator { nodes, config }
    }

    /// It creates an instance
    ///
    /// Arguments:
    ///
    /// * `instance`: Instance - the instance to create
    /// * `tx`: mpsc::Sender<Result<InstanceStatus, tonic::Status>>
    ///
    /// Returns:
    ///
    /// A stream of InstanceStatus
    pub async fn create_instance(
        &mut self,
        instance: Instance,
        tx: mpsc::Sender<Result<InstanceStatus, tonic::Status>>,
    ) -> Result<Streaming<proto::agent::InstanceStatus>, OrchestratorError> {
        let mut instance_scheduled =
            InstanceScheduled::new(instance.id.clone(), instance, None, tx);

        // set instance status to Scheduling
        instance_scheduled
            .change_status(Status::Scheduling, None)
            .await
            .map_err(OrchestratorError::FromProxyError)?;

        // find best node for the instance
        let target_node_id = self.find_best_node(&instance_scheduled.instance)?;
        let target_node = self
            .nodes
            .get_mut(&target_node_id)
            .ok_or(OrchestratorError::NodeNotFound)?; // should never be None

        log::info!(
            "instance {:?} is assigned on node {:?}",
            instance_scheduled.instance.id,
            target_node_id
        );

        // create the instance on the node
        let stream = target_node
            .create_instance(instance_scheduled.instance.clone())
            .await
            .map_err(OrchestratorError::FromProxyError)?;

        // set instance status to Scheduled
        instance_scheduled
            .change_status(
                Status::Scheduled,
                Some(format!(
                    "Instance is scheduled on node: {:?}",
                    target_node_id
                )),
            )
            .await
            .map_err(OrchestratorError::FromProxyError)?;

        // save the instance in the orchestrator
        target_node
            .instances
            .update(instance_scheduled.id.clone().as_str(), instance_scheduled);

        Ok(stream)
    }

    /// It stop an instance
    ///
    /// Arguments:
    ///
    /// * `id`: InstanceIdentifier
    ///
    /// Returns:
    ///
    /// A Result<(), OrchestratorError>
    pub async fn stop_instance(&mut self, id: InstanceIdentifier) -> Result<(), OrchestratorError> {
        // get the node that the instance is running on
        let node_registered = self
            .nodes
            .iter_mut()
            .filter(|(_, node)| node.instances.get_all().contains_key(id.clone().as_str()))
            .map(|(_, node)| node)
            .last()
            .ok_or(OrchestratorError::InstanceNotFound)?;

        // get the instance
        let instance_scheduled = node_registered
            .instances
            .get_mut(id.clone().as_str())
            .ok_or(OrchestratorError::InstanceNotFound)?;

        // check if instance is already stopped
        if instance_scheduled.instance.status() == Status::Stopped
            || instance_scheduled.instance.status() == Status::Stopping
        {
            return Ok(());
        }

        // send stop signal to the node
        node_registered
            .stop_instance(id)
            .await
            .map_err(OrchestratorError::FromProxyError)
    }

    /// It destroys an instance
    ///
    /// Arguments:
    ///
    /// * `id`: InstanceIdentifier
    ///
    /// Returns:
    ///
    /// A Result<(), OrchestratorError>
    pub async fn destroy_instance(
        &mut self,
        id: InstanceIdentifier,
    ) -> Result<(), OrchestratorError> {
        // get the node that the instance is running on
        let node_registered = self
            .nodes
            .iter_mut()
            .filter(|(_, node)| node.instances.get_all().contains_key(id.clone().as_str()))
            .map(|(_, node)| node)
            .last()
            .ok_or(OrchestratorError::InstanceNotFound)?;

        // get the instance
        let instance_scheduled = node_registered
            .instances
            .get_mut(id.clone().as_str())
            .ok_or(OrchestratorError::InstanceNotFound)?;

        // check if instance is already stopped
        if instance_scheduled.instance.status() == Status::Destroying
            || instance_scheduled.instance.status() == Status::Terminated
        {
            return Ok(());
        }

        // send kill signal to the node
        node_registered
            .kill_instance(id.clone())
            .await
            .map_err(OrchestratorError::FromProxyError)
    }

    /// It deletes an instance from the orchestrator
    ///
    /// Arguments:
    ///
    /// * `id`: InstanceIdentifier - The id of the instance to delete
    /// * `description`: A description of the reason for the instance termination.
    ///
    /// Returns:
    ///
    /// The result of the operation.
    pub async fn delete_instance(
        &mut self,
        id: InstanceIdentifier,
        description: Option<String>,
        change_status: bool,
    ) -> Result<(), OrchestratorError> {
        // get the node that the instance is running on
        let node_registered = self
            .nodes
            .iter_mut()
            .filter(|(_, node)| node.instances.get_all().contains_key(id.clone().as_str()))
            .map(|(_, node)| node)
            .last()
            .ok_or(OrchestratorError::InstanceNotFound)?;

        // get the instance
        let instance_scheduled = node_registered
            .instances
            .get_mut(id.clone().as_str())
            .ok_or(OrchestratorError::InstanceNotFound)?;

        if change_status {
            instance_scheduled
                .change_status(Status::Terminated, description)
                .await
                .map_err(OrchestratorError::FromProxyError)?;
        }

        instance_scheduled
            .tx
            .send(Err(tonic::Status::ok("")))
            .await
            .ok();
        node_registered.instances.delete(id.clone().as_str());
        Ok(())
    }

    /// It registers a node with the orchestrator
    ///
    /// Arguments:
    ///
    /// * `node`: Node - the node to register
    /// * `addr`: IpAddr - the IP address of the node
    ///
    /// Returns:
    ///
    /// Result<(), OrchestratorError>
    pub async fn register_node(
        &mut self,
        node: Node,
        addr: IpAddr,
        controller_client: Arc<Mutex<Option<NodeServiceClient<tonic::transport::Channel>>>>,
    ) -> Result<(), OrchestratorError> {
        let mut node_registered = NodeRegistered::new(node.id.clone(), node, addr);
        log::debug!("registering node: {:?}", node_registered);

        // connect to node agent grpc service
        node_registered
            .connect()
            .await
            .map_err(OrchestratorError::FromProxyError)?;

        node_registered
            .open_node_status_stream(controller_client)
            .await
            .map_err(OrchestratorError::FromProxyError)?;

        // update node status as Starting
        node_registered
            .update_status(Status::Starting, None, None)
            .await
            .map_err(OrchestratorError::FromProxyError)?;

        // save node in the orchestrator
        self.nodes
            .update(&node_registered.id.clone(), node_registered);
        Ok(())
    }

    /// This function unregister a node from the orchestrator
    ///
    /// Arguments:
    ///
    /// * `id`: The identifier of the node to be unregistered.
    /// * `description`: A description of the node.
    ///
    /// Returns:
    ///
    /// A Result<(), OrchestratorError>
    pub async fn unregister_node(
        &mut self,
        id: NodeIdentifier,
        description: Option<String>,
    ) -> Result<(), OrchestratorError> {
        // Return an error if the node is not found.
        let node_registered = self
            .nodes
            .get_mut(&id)
            .ok_or(OrchestratorError::NodeNotFound)?;

        // Update node status
        node_registered
            .update_status(Status::Terminated, description, None)
            .await
            .map_err(OrchestratorError::FromProxyError)?;

        // Update node's instances status
        for (_, instance) in node_registered.instances.iter_mut() {
            instance
                .change_status(Status::Terminated, Some("Node is terminated".to_string()))
                .await
                .map_err(OrchestratorError::FromProxyError)?;
        }

        node_registered.close_node_status_stream();
        self.nodes.delete(&id);
        Ok(())
    }

    /// Update the status of a node in the orchestrator.
    ///
    /// The first thing we do is get the node from the `nodes` map. If the node is not found, we return
    /// an error else we update the node status.
    ///
    /// Arguments:
    ///
    /// * `id`: The identifier of the node to update.
    /// * `status`: The status of the node.
    ///
    /// Returns:
    ///
    /// A Result<(), OrchestratorError>
    pub async fn update_node_status(
        &mut self,
        id: NodeIdentifier,
        status: NodeStatus,
    ) -> Result<(), OrchestratorError> {
        // Return an error if the node is not found.
        let node_registered = self
            .nodes
            .get_mut(&id.clone())
            .ok_or(OrchestratorError::NodeNotFound)?;

        node_registered
            .update_status(
                status.status(),
                Some(status.status_description),
                status.resource,
            )
            .await
            .map_err(OrchestratorError::FromProxyError)
    }

    /// Find the best node for the given instance
    ///
    /// Arguments:
    ///
    /// * `instance`: The instance that we want to find a node for.
    ///
    /// Returns:
    ///
    /// A Result<NodeIdentifier, OrchestratorError>
    pub fn find_best_node(&self, instance: &Instance) -> Result<NodeIdentifier, OrchestratorError> {
        log::debug!("Finding best node for instance: {:?}", instance);

        let nodes = self.nodes.get_all();

        let started_nodes: Vec<(_, &NodeRegistered)> = nodes
            .iter()
            .filter(|node| node.1.node.status == Status::Running)
            .collect();

        if started_nodes.is_empty() {
            return Err(OrchestratorError::NoAvailableNodes);
        }

        let mut best_node_id: Option<String> = None;
        let mut best_node_score: f64 = 0.0;

        for (_, node_registered) in started_nodes {
            let node_score = match self
                .score_node_for_an_new_instance(node_registered.node.clone(), instance.clone())
            {
                Ok(score) => score,
                Err(_) => continue,
            };

            if node_score > best_node_score {
                best_node_score = node_score;
                best_node_id = Some(node_registered.id.clone());
            }
        }

        best_node_id.ok_or(OrchestratorError::NotEnoughResources)
    }

    /// It takes a node and an instance and returns a score for the node based on the resources of the
    /// instance and the resources of the node. The score is a float between 0 and 1, with 1 being
    /// the best score.
    ///
    /// Arguments:
    ///
    /// * `node`: Node,
    /// * `instance`: Instance
    ///
    /// Returns:
    ///
    /// The score of the node for the new instance.
    fn score_node_for_an_new_instance(
        &self,
        node: Node,
        instance: Instance,
    ) -> Result<f64, OrchestratorError> {
        let instances_scheduled = self
            .nodes
            .get(&node.id)
            .ok_or(OrchestratorError::NodeNotFound)?;

        let mut sum_instances_resource_limit: ResourceSummary = match instance.resource {
            Some(resource) => match resource.limit {
                Some(limit) => ResourceSummary {
                    cpu: limit.cpu,
                    memory: limit.memory,
                    disk: limit.disk,
                },
                None => {
                    return Err(OrchestratorError::InvalidWorkload);
                }
            },
            None => return Err(OrchestratorError::InvalidWorkload),
        };

        for instance_proxied in instances_scheduled.instances.get_all().values() {
            if let Some(limit) = instance_proxied
                .instance
                .resource
                .clone()
                .unwrap_or_default()
                .limit
            {
                sum_instances_resource_limit.cpu += limit.cpu;
                sum_instances_resource_limit.memory += limit.memory;
                sum_instances_resource_limit.disk += limit.disk;
            }
        }

        let node_resource_summary_limit = match node.resource {
            Some(resource) => match resource.limit {
                Some(limit) => ResourceSummary {
                    cpu: limit.cpu,
                    memory: limit.memory,
                    disk: limit.disk,
                },
                None => {
                    return Err(OrchestratorError::InvalidWorkload);
                }
            },
            None => return Err(OrchestratorError::InvalidWorkload),
        };

        let mut node_score = 0.0;

        if Self::has_enough_resources(
            node_resource_summary_limit.clone(),
            sum_instances_resource_limit.clone(),
        ) {
            let cpu_score =
                sum_instances_resource_limit.cpu as f64 / node_resource_summary_limit.cpu as f64;

            let memory_score = sum_instances_resource_limit.memory as f64
                / node_resource_summary_limit.memory as f64;

            let disk_score =
                sum_instances_resource_limit.disk as f64 / node_resource_summary_limit.disk as f64;

            node_score = 1.0 - (cpu_score + memory_score + disk_score) / 3.0;
        }

        Ok(node_score)
    }

    /// It takes a `Resource` struct, and returns a `ResourceSummary` struct
    ///
    /// Arguments:
    ///
    /// * `resource`: Resource - The resource object that we're going to compute the available resources
    /// for.
    ///
    /// Returns:
    ///
    /// A Result<ResourceSummary, OrchestratorError>
    #[allow(dead_code)]
    fn compute_available_resources(
        resource: Resource,
    ) -> Result<ResourceSummary, OrchestratorError> {
        let available_limit = resource
            .limit
            .ok_or(OrchestratorError::NotEnoughResources)?;

        let available_usage = resource
            .usage
            .ok_or(OrchestratorError::NotEnoughResources)?;

        Ok(ResourceSummary {
            cpu: available_limit.cpu - available_usage.cpu,
            memory: available_limit.memory - available_usage.memory,
            disk: available_limit.disk - available_usage.disk,
        })
    }

    /// "Returns true if the total resources limits of the instances are less than 95%
    /// of the node's resources limit"
    ///
    /// Arguments:
    ///
    /// * `available_resources`: ResourceSummary - The available resources on the node
    /// * `needed_resources`: ResourceSummary - The resources that the user wants to use
    ///
    /// Returns:
    ///
    /// A boolean value.
    fn has_enough_resources(
        available_resources: ResourceSummary,
        needed_resources: ResourceSummary,
    ) -> bool {
        available_resources.cpu as f64 * 0.95 >= needed_resources.cpu as f64
            && available_resources.memory as f64 * 0.95 >= needed_resources.memory as f64
            && available_resources.disk as f64 * 0.95 >= needed_resources.disk as f64
    }
}

#[cfg(test)]
mod tests {
    use std::{
        net::{IpAddr, Ipv4Addr},
        sync::Arc,
    };

    use proto::scheduler::{Instance, Resource, ResourceSummary, Status};

    use crate::{
        config::Config,
        node::{registered::NodeRegistered, Node},
        storage::{IStorage, Storage},
    };

    use super::Orchestrator;

    #[test]
    fn find_best_node_enough_resources() {
        let mut nodes: Storage<NodeRegistered> = Storage::new();
        let config = Arc::new(Config::default());

        nodes.update(
            "node",
            NodeRegistered {
                id: "node".to_string(),
                node: Node {
                    id: "node".to_string(),
                    status: Status::Running,
                    resource: Some(Resource {
                        limit: Some(ResourceSummary {
                            cpu: 20,
                            memory: 20,
                            disk: 20,
                        }),
                        usage: None,
                    }),
                },
                address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                tx: None,
                grpc_client: None,
                status_thread: None,
                instances: Storage::new(),
            },
        );

        nodes.update(
            "node2",
            NodeRegistered {
                id: "node2".to_string(),
                node: Node {
                    id: "node2".to_string(),
                    status: Status::Running,
                    resource: Some(Resource {
                        limit: Some(ResourceSummary {
                            cpu: 50,
                            memory: 50,
                            disk: 50,
                        }),
                        usage: None,
                    }),
                },
                address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                tx: None,
                grpc_client: None,
                status_thread: None,
                instances: Storage::new(),
            },
        );

        let orchestrator = Orchestrator::new(nodes, config);

        let instance: Instance = Instance {
            id: "instance".to_string(),
            name: "instance".to_string(),
            r#type: 0,
            status: 7,
            uri: "127.0.0.1".to_string(),
            environnement: Vec::new(),
            resource: Some(Resource {
                limit: Some(ResourceSummary {
                    cpu: 30,
                    memory: 10,
                    disk: 5,
                }),
                usage: None,
            }),
            ports: Vec::new(),
            ip: "127.0.0.1".to_string(),
        };

        let node_id = orchestrator.find_best_node(&instance).unwrap();

        assert_eq!(node_id, "node2");
    }

    #[test]
    fn find_best_node_not_enough_resources() {
        let mut nodes: Storage<NodeRegistered> = Storage::new();
        let config = Arc::new(Config::default());

        nodes.update(
            "node",
            NodeRegistered {
                id: "node".to_string(),
                node: Node {
                    id: "node".to_string(),
                    status: Status::Running,
                    resource: Some(Resource {
                        limit: Some(ResourceSummary {
                            cpu: 20,
                            memory: 20,
                            disk: 20,
                        }),
                        usage: None,
                    }),
                },
                address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                tx: None,
                grpc_client: None,
                status_thread: None,
                instances: Storage::new(),
            },
        );

        nodes.update(
            "node2",
            NodeRegistered {
                id: "node2".to_string(),
                node: Node {
                    id: "node2".to_string(),
                    status: Status::Running,
                    resource: Some(Resource {
                        limit: Some(ResourceSummary {
                            cpu: 50,
                            memory: 50,
                            disk: 50,
                        }),
                        usage: None,
                    }),
                },
                address: IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                tx: None,
                grpc_client: None,
                status_thread: None,
                instances: Storage::new(),
            },
        );

        let instance: Instance = Instance {
            id: "instance".to_string(),
            name: "instance".to_string(),
            r#type: 0,
            status: 7,
            uri: "127.0.0.1".to_string(),
            environnement: Vec::new(),
            resource: Some(Resource {
                limit: Some(ResourceSummary {
                    cpu: 30,
                    memory: 48,
                    disk: 5,
                }),
                usage: None,
            }),
            ports: Vec::new(),
            ip: "127.0.0.1".to_string(),
        };

        if Orchestrator::new(nodes, config)
            .find_best_node(&instance)
            .is_ok()
        {
            panic!("should not find a node");
        }
    }

    #[test]
    fn find_best_node_no_nodes_available() {
        let nodes: Storage<NodeRegistered> = Storage::new();
        let config = Arc::new(Config::default());

        let instance: Instance = Instance {
            id: "instance".to_string(),
            name: "instance".to_string(),
            r#type: 0,
            status: 7,
            uri: "127.0.0.1".to_string(),
            environnement: Vec::new(),
            resource: Some(Resource {
                limit: Some(ResourceSummary {
                    cpu: 0,
                    memory: 0,
                    disk: 0,
                }),
                usage: None,
            }),
            ports: Vec::new(),
            ip: "127.0.0.1".to_string(),
        };

        if Orchestrator::new(nodes, config)
            .find_best_node(&instance)
            .is_ok()
        {
            panic!("should not find a node");
        }
    }
}
