use std::net::Ipv4Addr;

use cidr::Ipv4Inet;

// Setup
pub struct SetupNodeRequest {
    /// Unique identifier of the node. This identifier is used to create a bridge interface
    pub node_id: String,
    /// Composed of the node ip and a mask
    pub node_ip_cidr: Ipv4Inet,
}

impl SetupNodeRequest {
    pub fn new(node_id: String, node_ip_cidr: Ipv4Inet) -> Self {
        Self {
            node_id,
            node_ip_cidr,
        }
    }
}

pub struct SetupIptablesRequest {
    pub node_id: String,
}

impl SetupIptablesRequest {
    pub fn new(node_id: String) -> Self {
        Self { node_id }
    }
}

pub struct NodeRequest {
    pub node_id: String,
    pub node_ip_cidr: Ipv4Inet,
    pub node_public_ip: Ipv4Addr,
}
// Clean up
pub struct CleanNodeRequest {
    pub node_id: String,
}

impl CleanNodeRequest {
    pub fn new(node_id: String) -> Self {
        Self { node_id }
    }
}
