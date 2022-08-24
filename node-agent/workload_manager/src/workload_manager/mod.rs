use std::string::ToString;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, channel};

mod workload;
use workload::Workload;

mod workload_listener;
use workload_listener::WorkloadListener;

use proto::agent::{
    Instance,
    InstanceStatus,
    SignalInstruction,
};

use tonic::Status; 

pub struct WorkloadManager {
    workloads: HashMap<String, Workload>,
    listeners: HashMap<String, WorkloadListener>,
    tx_rx: (Sender<InstanceStatus>, Receiver<InstanceStatus>) 
}

impl Default for WorkloadManager {
    fn default() -> Self {
        Self { 
            workloads: Default::default(), 
            listeners: Default::default(), 
            tx_rx: channel() 
        }
    }
}

impl WorkloadManager {

    pub fn new() -> Self {
        WorkloadManager::default()
    }

    pub async fn create(&mut self, instance: Instance) -> Result<&Receiver<InstanceStatus>, Status> {
        let (tx, rx) = &self.tx_rx;
        
        let workload_id = instance.clone().id;
        //Create a workload, it's listener and an id which will be an UUID
        let workload = Workload::create(instance).await;

        //create listener from the workloadId;
        //let listener = WorkloadListener::new(workload.get_id(), instance, tx.clone());

        self.workloads.insert(workload_id, workload);
        //self.listeners.insert(workload_id, listener);
        
        //add match here to handle errors on listener and workload creation
        Ok(rx)
        
//        Err(WorkloadError::new("Error"))
    }

    pub async fn signal(&mut self, signal_instruction: SignalInstruction) -> Result<(), Status> {
        
        let workload_id = match signal_instruction.instance {
            Some(inst) => inst.id, 
            None => return Err(
                Status::invalid_argument(
                    "Please provide an 'Instance'"
            ))
        };

        let workload = match self.workloads.get(&workload_id.clone()) {
            None => return Err(Status::not_found("This workload does not exist")),
            Some(wrkld) => wrkld
        };

        match signal_instruction.signal {
            0 => workload.stop().await, // Status::Stop
            1 => workload.kill().await, // Status::Kill
            _ => return Err(Status::not_found("This signal does not exist")) 
        };

        self.workloads.remove(&workload_id);

        Ok(())
    } 
}

impl ToString for WorkloadManager {
    fn to_string(&self) -> String {
        format!("workloads: {}, listeners: {}", self.workloads.len(), self.listeners.len())
    }
}