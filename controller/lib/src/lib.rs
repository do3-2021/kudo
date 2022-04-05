use log::debug;

pub mod external_api;
pub mod internal_api;
pub mod controller {
    tonic::include_proto!("controller");
}
