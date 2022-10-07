use serde::{Deserialize, Serialize};
use std::net::{Ipv4Addr, SocketAddr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KudoControllerConfig {
    pub internal_api: InternalAPIConfig,
    pub external_api: ExternalAPIConfig,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct InternalAPIConfig {
    pub grpc_server_addr: SocketAddr,
    pub grpc_client_addr: String,
    pub grpc_client_connection_max_retries: u32,
    pub time_after_node_erased: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalAPIConfig {
    pub http_server_addr: SocketAddr,
    pub http_server_num_workers: usize,
    pub etcd_address: SocketAddr,
    pub grpc_client_connection_max_retries: u32,
}

impl Default for KudoControllerConfig {
    fn default() -> Self {
        KudoControllerConfig {
            internal_api: InternalAPIConfig {
                grpc_server_addr: SocketAddr::new(
                    std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    50051,
                ),
                grpc_client_addr: "http://127.0.0.1:50052".to_string(),
                grpc_client_connection_max_retries: 32,
                time_after_node_erased: 350,
            },
            external_api: ExternalAPIConfig {
                http_server_addr: SocketAddr::new(
                    std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    3000,
                ),
                http_server_num_workers: 1,
                etcd_address: SocketAddr::new(
                    std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    2379,
                ),
                grpc_client_connection_max_retries: 32,
            },
        }
    }
}
