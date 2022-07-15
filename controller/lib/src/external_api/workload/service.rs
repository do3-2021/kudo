use super::model::{Workload, WorkloadError, Type, Ressources};
use uuid::Uuid;

static mut WORKLOADS: Vec<Workload> = Vec::new();

/* remove this function when etcd is implemented */
pub fn get_workload(workload_id: &str) -> Result<Workload, WorkloadError> {
   unsafe {
    return match WORKLOADS.iter().find(|w| w.id == workload_id) {
            Some(workload) => Ok(workload.clone()),
            None => Err(WorkloadError::WorkloadNotFound),
    }
   }
}

pub fn get_all_workloads() -> Vec<Workload> {
    unsafe {
        return WORKLOADS.clone();
    }
}

pub fn create_workload(name : String, environment : &[String], port : &[String] ) {
    let workload = Workload {
        id: Uuid::new_v4().to_string(),
        name: name,
        workload_type: Type::CONTAINER,
        uri: "http://localhost:8080".to_string(),
        environment: environment.to_vec(),
        resources: Ressources {
            cpu: 0,
            memory: 0,
            disk: 0
        },
        ports: port.to_vec()
    };
    // remove push
    unsafe {
        WORKLOADS.push(workload);
    }
}


/* 
pub fn delete_workload(id: &String) -> String {
    return format!("delete_workload {}", id);
}*/
