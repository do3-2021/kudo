use std::fmt::Error;

use proto::controller::NodeStatus;

pub fn update_node_status(_node_status: NodeStatus) -> Result<(), Error> {
    return Ok(());
}
