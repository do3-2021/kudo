use std::collections::HashMap;

use bollard::container::{
    Config, KillContainerOptions, NetworkingConfig, RemoveContainerOptions, RenameContainerOptions,
    StopContainerOptions,
};
use bollard::Docker;

use anyhow::{Context, Error, Result};

use bollard::image::CreateImageOptions;
use bollard::service::EndpointSettings;
use futures_util::TryStreamExt;

use super::workload_runner::NetworkSettings;
use super::workload_trait::Workload;
use proto::agent::Instance;

pub struct Container {
    id: String,
}

impl Container {
    /// Create a new workload (container) and start it
    pub async fn new(
        instance: Instance,
        network_settings: &NetworkSettings,
    ) -> Result<Self, Error> {
        let docker =
            Docker::connect_with_socket_defaults().context("Can't connect to docker socket. ")?;

        docker
            .create_image(
                Some(CreateImageOptions {
                    from_image: instance.uri.clone(),
                    ..Default::default()
                }),
                None,
                None,
            )
            .try_collect::<Vec<_>>()
            .await
            .context("Can't create image. ")?;

        let container_id = create_container(&docker, instance, network_settings).await?;

        docker
            .start_container::<String>(container_id.as_str(), None)
            .await
            .context("Can't start container. ")?;

        Ok(Container { id: container_id })
    }

    /// Removes a container
    async fn remove(&self) -> Result<(), Error> {
        let docker =
            Docker::connect_with_socket_defaults().context("Can't connect to docker socket. ")?;
        docker
            .remove_container(
                self.id().as_str(),
                Some(RemoveContainerOptions {
                    force: true,
                    ..Default::default()
                }),
            )
            .await
            .context("Can't remove container. ")?;

        Ok(())
    }
}

#[tonic::async_trait]
impl Workload for Container {
    fn id(&self) -> String {
        self.id.to_string()
    }

    // Gracefully stop a workload
    async fn stop(&self) -> Result<(), Error> {
        let docker =
            Docker::connect_with_socket_defaults().context("Can't connect to docker socket. ")?;

        docker
            .stop_container(
                self.id().as_str(),
                Some(StopContainerOptions {
                    ..Default::default()
                }),
            )
            .await
            .context("Can't stop docker container.")?;

        self.remove().await?;

        Ok(())
    }

    // Force a workload to stop
    // (equivalent to a `kill -9` on linux)
    async fn kill(&self) -> Result<(), Error> {
        let docker =
            Docker::connect_with_socket_defaults().context("Can't connect to docker socket. ")?;

        docker
            .kill_container(
                self.id().as_str(),
                Some(KillContainerOptions { signal: "SIGKILL" }),
            )
            .await
            .context("Can't kill docker container. ")?;

        self.remove().await?;

        Ok(())
    }
}

/// It creates a container with the given instance's configuration
///
/// Arguments:
///
/// * `docker`: &Docker - This is the docker client that we created earlier.
/// * `instance`: Instance
///
/// Returns:
///
/// A string that is the container id.
async fn create_container(
    docker: &Docker,
    instance: Instance,
    network_settings: &NetworkSettings,
) -> Result<String> {
    let mut ports = HashMap::new();
    let port_list = &instance.ports;
    for port in port_list {
        ports.insert(
            port.destination.to_string(),
            Some(vec![bollard::service::PortBinding {
                host_port: Some(port.source.to_string()),
                ..Default::default()
            }]),
        );
    }

    let random_id = uuid::Uuid::new_v4().to_string();

    let container_config: Config<&str> = Config {
        image: Some(instance.uri.as_str()),
        tty: Some(true),
        host_config: Some(bollard::service::HostConfig {
            port_bindings: Some(ports),
            nano_cpus: instance
                .resource
                .clone()
                .and_then(|resource| resource.limit.map(|limit| limit.cpu.try_into().unwrap())),
            memory: instance
                .resource
                .clone()
                .and_then(|resource| resource.limit.map(|limit| limit.memory.try_into().unwrap())),

            ..Default::default()
        }),
        networking_config: Some(NetworkingConfig {
            endpoints_config: HashMap::from([(
                random_id.as_str(),
                EndpointSettings {
                    links: Some(vec![network_settings.bridge_name.clone()]),
                    ip_address: Some(instance.ip),
                    ..Default::default()
                },
            )]),
        }),
        ..Default::default()
    };

    let container_id = docker
        .create_container::<&str, &str>(None, container_config)
        .await
        .context("Can't create container. ")?
        .id;

    docker
        .rename_container(
            container_id.as_str(),
            RenameContainerOptions {
                name: instance.name,
            },
        )
        .await
        .ok();

    Ok(container_id)
}

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::workload_manager::workload::{
        workload_runner::NetworkSettings, workload_trait::Workload,
    };

    use super::Container;
    use anyhow::{Error, Result};
    use bollard::{container::ListContainersOptions, Docker};
    use cidr::Ipv4Inet;
    use network::node::{
        clean_node,
        request::{CleanNodeRequest, SetupNodeRequest},
        setup_node,
    };
    use proto::agent::{Instance, Resource, ResourceSummary, Type};

    const IMAGE: &str = "alpine:3";

    /// Creates a container used for test
    ///
    /// Arguments:
    ///
    /// * `node_id`: The ID of the node that the container is running on.
    ///
    /// Returns:
    ///
    /// A container
    async fn create_default_container(node_id: String) -> Result<Container, Error> {
        let resource: Resource = Resource {
            limit: Some(ResourceSummary {
                cpu: 0,
                memory: 0,
                disk: 0,
            }),
            usage: Some(ResourceSummary {
                cpu: 0,
                memory: 0,
                disk: 0,
            }),
        };

        let instance_id = node_id.clone();

        let instance = Instance {
            id: instance_id,
            name: "test_container".to_string(),
            ip: "127.0.0.1/32".to_string(),
            uri: IMAGE.to_string(),
            environment: Vec::new(),
            ports: Vec::new(),
            resource: Some(resource),
            status: 1,
            r#type: Type::Container.into(),
        };

        Ok(Container::new(
            instance,
            &NetworkSettings {
                node_id: node_id.clone(),
                bridge_name: node_id,
            },
        )
        .await?)
    }

    /// It sets up a node, runs a test, and cleans up the node
    ///
    /// Arguments:
    ///
    /// * `test`: The test function to run.
    /// * `node_id`: The name of the node to be created.
    fn run_container<F: std::future::Future>(test: F, node_id: String) {
        let mut nb_retry = 5;

        loop {
            match setup_node(SetupNodeRequest::new(
                node_id.to_string(),
                Ipv4Inet::from_str("127.0.0.1/32").unwrap(),
            )) {
                Ok(_) => break,
                Err(e) => {
                    if nb_retry <= 0 {
                        panic!("{:?}", e)
                    }
                    clean_node(CleanNodeRequest::new(node_id.to_string())).ok();
                    nb_retry -= 1;
                    std::thread::sleep(core::time::Duration::from_millis(1000));
                }
            };
        }

        tokio_test::block_on(test);

        clean_node(CleanNodeRequest::new(node_id.to_string())).ok();
    }

    /// It creates a container, checks that it exists, and then stops it
    ///
    /// Arguments:
    ///
    /// * `node_id`: The id of the node to create the container on.
    ///
    /// Returns:
    ///
    /// A Result<(), Error>
    async fn create_container_test(node_id: String) -> Result<(), Error> {
        let container = create_default_container(node_id).await?;

        let docker = Docker::connect_with_socket_defaults()?;

        let result = &docker
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await?;

        assert_ne!(0, result.len());
        assert!(result
            .iter()
            .any(|cont| cont.id.as_ref().unwrap() == &container.id),);

        container.stop().await.unwrap();

        Ok(())
    }

    /// It creates a container, stops it, and then checks that it's no longer in the list of containers
    ///
    /// Arguments:
    ///
    /// * `node_id`: The id of the node to run the test on.
    ///
    /// Returns:
    ///
    /// A Result<(), Error>
    async fn stop_container_test(node_id: String) -> Result<(), Error> {
        let docker = Docker::connect_with_socket_defaults()?;

        let container = create_default_container(node_id).await?;

        container.stop().await?;

        let result = &docker
            .list_containers(Some(ListContainersOptions::<String> {
                all: true,
                ..Default::default()
            }))
            .await?;

        assert_eq!(
            result
                .iter()
                .any(|cont| cont.id.as_ref().unwrap() == &container.id),
            false
        );

        Ok(())
    }

    #[test]
    fn test_create_container() {
        let name = "test_create";
        run_container(create_container_test(name.to_string()), name.to_string());
    }

    #[test]
    fn test_stop_container() {
        let name = "test_stop";
        run_container(stop_container_test(name.to_string()), name.to_string());
    }
}
