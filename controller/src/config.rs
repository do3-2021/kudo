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
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ExternalAPIConfig {
    pub http_server_addr: SocketAddr,
    pub http_server_num_workers: usize,
}

impl Default for KudoControllerConfig {
    fn default() -> Self {
        KudoControllerConfig {
            internal_api: InternalAPIConfig {
                grpc_server_addr: SocketAddr::new(
                    std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    50051,
                ),
            },
            external_api: ExternalAPIConfig {
                http_server_addr: SocketAddr::new(
                    std::net::IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
                    3000,
                ),
                http_server_num_workers: 1,
            },
        }
    }
}
