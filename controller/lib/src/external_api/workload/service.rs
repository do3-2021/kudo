use super::model::WorkloadInfo;

pub fn get_workloads(ids: &Vec<String>) -> String {
    return format!("get_workloads {}", ids.join(","));
}

pub fn create_workload(workload: &WorkloadInfo) -> String {
    return format!("create_workload {}", workload.name);
}

pub fn update_workload(id: &String, workload: &WorkloadInfo) -> String {
    return format!("update_workload {}, id {}", workload.name, id);
}

pub fn delete_workload(id: &String) -> String {
    return format!("delete_workload {}", id);
}
