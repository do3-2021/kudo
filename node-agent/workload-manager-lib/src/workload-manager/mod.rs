use std::string::ToString;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, channel};

mod workload;
use workload::error::{WorkloadError};
use workload::Workload;

#[path="./workload-listener/mod.rs"]
mod workload_listener;
use workload_listener::WorkloadListener;
use proto::agent::{Instance, SignalInstruction, Signal};

use uuid::Uuid;

pub struct WorkloadManager {
    workloads: HashMap<String, Workload>,
    listeners: HashMap<String, WorkloadListener>,
    tx_rx: (Sender<InstanceStatus>, Receiver<InstanceStatus>) 
}

impl WorkloadManager {
    pub fn new() -> Self {
        WorkloadManager{ 
            tx_rx: channel(),
            workloads: HashMap::new(), 
            listeners: HashMap::new() 
        }
    }

    pub async fn create(&mut self, instance: Instance) -> Result<Receiver, WorkloadManagerError> {
        let id = Uuid::new_v4();
        (tx, rx) = self.tx_rx;

        //Create a workload, it's listener and an id which will be an UUID
        let workload = Workload::create(instance).await;

        //let listener_id = String::from(id.clone()); 
        //create listener from the workloadId;
        //let listener = Listener::new(workload.get_id(), tx.clone()).await;

        self.workloads.insert(id.clone(), workload);
        //self.listeners.insert(listener_id, listener);
        
        //add match here to handle errors on listener and workload creation
        Ok(rx)
        
//        Err(WorkloadError::new("Error"))
    }

    pub async fn signal(&mut self, signalInstruction: SignalInstruction) -> Result<(), WorkloadError> {
        let workload_id = signalInstruction.instance.id;

        let workload = match(self.workloads.get(workload_id.clone())) {
            None => Err(WorkloadError::new("This workload does not exist")),
            Some(workload) => workload
        };

        match(signalInstruction.signal) {
            Signal::STOP => workload.stop().await,
            Signal::KILL => workload.kill().await,
            _ => Err(WorkloadError::new("This signal does not exist")) 
        };

        self.workloads.remove(workload_id);

        Ok(())
    } 
}

impl ToString for WorkloadManager {
    fn to_string(&self) -> String {
        format!("workloads: {}, listeners: {}", self.workloads.len(), self.listeners.len())
    }
}