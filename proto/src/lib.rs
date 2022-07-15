pub mod network {
    tonic::include_proto!("network");
}

pub mod controller {
    tonic::include_proto!("controller");
}

pub mod agent {
    tonic::include_proto!("agent");
}

pub mod scheduler {
    tonic::include_proto!("scheduler");
}
