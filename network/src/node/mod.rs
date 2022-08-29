pub mod request;
pub mod response;

use crate::error::KudoNetworkError;
use crate::utils::{bridge_name, run_command};
use default_net;
use request::{CleanNodeRequest, SetupIptablesRequest, SetupNodeRequest};
use response::SetupNodeResponse;

use self::request::NodeRequest;

/// Create a network interface and add iptables rules to make this device able to route instances
/// traffic
pub fn setup_node(request: SetupNodeRequest) -> Result<SetupNodeResponse, KudoNetworkError> {
    let bridge = bridge_name(request.node_id.clone());

    run_command("ip", &["link", "add", "dev", &bridge, "type", "bridge"])?;

    run_command(
        "ip",
        &[
            "addr",
            "add",
            &request.node_ip_cidr.to_string(),
            "dev",
            &bridge,
        ],
    )?;
    run_command("ip", &["link", "set", "dev", &bridge, "up"])?;

    enable_ip_forward()?;
    enable_route_localnet(&bridge)?;

    setup_iptables(SetupIptablesRequest::new(request.node_id))?;

    Ok(SetupNodeResponse::new(bridge))
}

/// Add iptables rules when a node joins the cluster
pub fn new_node_in_cluster(request: NodeRequest) -> Result<(), KudoNetworkError> {
    // Iptables rules to allow workloads to route to the node
    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-I",
            "PREROUTING",
            "-p",
            "tcp",
            "-d",
            &request.node_ip_cidr.to_string(),
            "-j",
            "DNAT",
            "--to-destination",
            &request.node_public_ip.to_string(),
        ],
    )?;

    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-I",
            "PREROUTING",
            "-p",
            "udp",
            "-d",
            &request.node_ip_cidr.to_string(),
            "-j",
            "DNAT",
            "--to-destination",
            &request.node_public_ip.to_string(),
        ],
    )?;

    // Iptables rules to allow host machines to route to the node
    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-I",
            "OUTPUT",
            "-p",
            "tcp",
            "-d",
            &request.node_ip_cidr.to_string(),
            "-j",
            "DNAT",
            "--to-destination",
            &request.node_public_ip.to_string(),
        ],
    )?;

    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-I",
            "OUTPUT",
            "-p",
            "udp",
            "-d",
            &request.node_ip_cidr.to_string(),
            "-j",
            "DNAT",
            "--to-destination",
            &request.node_public_ip.to_string(),
        ],
    )?;

    Ok(())
}

/// Remove iptables rules when a node leaves the cluster
pub fn delete_node_in_cluster(request: NodeRequest) -> Result<(), KudoNetworkError> {
    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-D",
            "PREROUTING",
            "-p",
            "tcp",
            "-d",
            &request.node_ip_cidr.to_string(),
            "-j",
            "DNAT",
            "--to-destination",
            &request.node_public_ip.to_string(),
        ],
    )?;

    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-D",
            "PREROUTING",
            "-p",
            "udp",
            "-d",
            &request.node_ip_cidr.to_string(),
            "-j",
            "DNAT",
            "--to-destination",
            &request.node_public_ip.to_string(),
        ],
    )?;

    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-D",
            "OUTPUT",
            "-p",
            "tcp",
            "-d",
            &request.node_ip_cidr.to_string(),
            "-j",
            "DNAT",
            "--to-destination",
            &request.node_public_ip.to_string(),
        ],
    )?;

    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-D",
            "OUTPUT",
            "-p",
            "udp",
            "-d",
            &request.node_ip_cidr.to_string(),
            "-j",
            "DNAT",
            "--to-destination",
            &request.node_public_ip.to_string(),
        ],
    )?;

    Ok(())
}

/// Add iptables rules to route instances traffic
/// This function must be called after each reboot
pub fn setup_iptables(request: SetupIptablesRequest) -> Result<(), KudoNetworkError> {
    let bridge = bridge_name(request.node_id);
    let default_interface = match default_net::get_default_interface() {
        Ok(default_interface) => Ok(default_interface.name),
        Err(e) => Err(KudoNetworkError::DefaultNetworkInterfaceError(format!(
            "Could not find default interface : {}",
            e
        ))),
    }?;

    run_command(
        "iptables",
        &[
            "-A",
            "POSTROUTING",
            "-t",
            "nat",
            "-o",
            &default_interface,
            "-j",
            "MASQUERADE",
        ],
    )?;
    // We need an iptables rule to allow traffic crossing the bridge itself
    run_command(
        "iptables",
        &[
            "-A", "FORWARD", "-i", &bridge, "-o", &bridge, "-j", "ACCEPT",
        ],
    )?;
    run_command(
        "iptables",
        &[
            "-A",
            "FORWARD",
            "-i",
            &bridge,
            "-o",
            &default_interface,
            "-j",
            "ACCEPT",
        ],
    )?;
    run_command(
        "iptables",
        &[
            "-A",
            "FORWARD",
            "-i",
            &default_interface,
            "-o",
            &bridge,
            "-j",
            "ACCEPT",
        ],
    )?;
    // Needed to route packets coming from localhost
    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-A",
            "POSTROUTING",
            "-o",
            &bridge,
            "-m",
            "addrtype",
            "--src-type",
            "LOCAL",
            "--dst-type",
            "UNICAST",
            "-j",
            "MASQUERADE",
        ],
    )?;

    Ok(())
}

/// Remove node network interface and its iptables rules
pub fn clean_node(request: CleanNodeRequest) -> Result<(), KudoNetworkError> {
    let bridge = bridge_name(request.node_id);
    let default_interface = match default_net::get_default_interface() {
        Ok(default_interface) => Ok(default_interface.name),
        Err(e) => Err(KudoNetworkError::DefaultNetworkInterfaceError(format!(
            "Could not find default interface : {}",
            e
        ))),
    }?;
    run_command("ip", &["link", "del", &bridge])?;
    run_command(
        "iptables",
        &[
            "-D",
            "POSTROUTING",
            "-t",
            "nat",
            "-o",
            &default_interface,
            "-j",
            "MASQUERADE",
        ],
    )?;
    run_command(
        "iptables",
        &[
            "-D", "FORWARD", "-i", &bridge, "-o", &bridge, "-j", "ACCEPT",
        ],
    )?;
    run_command(
        "iptables",
        &[
            "-D",
            "FORWARD",
            "-i",
            &bridge,
            "-o",
            &default_interface,
            "-j",
            "ACCEPT",
        ],
    )?;
    run_command(
        "iptables",
        &[
            "-D",
            "FORWARD",
            "-i",
            &default_interface,
            "-o",
            &bridge,
            "-j",
            "ACCEPT",
        ],
    )?;
    run_command(
        "iptables",
        &[
            "-t",
            "nat",
            "-D",
            "POSTROUTING",
            "-o",
            &bridge,
            "-m",
            "addrtype",
            "--src-type",
            "LOCAL",
            "--dst-type",
            "UNICAST",
            "-j",
            "MASQUERADE",
        ],
    )?;
    Ok(())
}

fn enable_ip_forward() -> Result<(), KudoNetworkError> {
    run_command("sysctl", &["-w", "net.ipv4.ip_forward=1"]).map_err(|err| {
        KudoNetworkError::IPForwardError(format!("Failed to enable net.ipv4.ip_forward ({})", err))
    })?;
    Ok(())
}

fn enable_route_localnet(bridge_name: &str) -> Result<(), KudoNetworkError> {
    run_command(
        "sysctl",
        &[
            "-w",
            &format!("net.ipv4.conf.{}.route_localnet=1", bridge_name),
        ],
    )
    .map_err(|err| {
        KudoNetworkError::RouteLocalnetError(format!(
            "Failed to enable net.ipv4.conf.{}.route_localnet ({})",
            bridge_name, err
        ))
    })?;
    Ok(())
}
