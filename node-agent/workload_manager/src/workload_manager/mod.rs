use std::string::ToString;
use std::collections::HashMap;
use std::sync::mpsc::{Sender, Receiver, channel};

mod workload;
use tonic::codegen::http::status;
use workload::Workload;

mod workload_listener;
use workload_listener::WorkloadListener;

use proto::agent::{
    Instance,
    InstanceStatus,
    SignalInstruction,
    Status as WorkloadStatus
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

    /// Creates an empty WorkloadManager
    pub fn new() -> Self {
        WorkloadManager::default()
    }

    /// Creates a Workload, run it and starts its listener, a receiver is returned to read all Workloads' status
    ///
    /// # Arguments 
    /// * `instance` - Respresentation of instance to create 
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

    /// Send a signal to a Workload
    ///
    /// # Arguments 
    /// * `signal_instruction` - Respresentation of signal to send 
    pub async fn signal(&mut self, signal_instruction: SignalInstruction) -> Result<(), Status> {
        let tx = &self.tx_rx.0;

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

        let status_stopping = InstanceStatus {
            id: workload_id.clone(),
            status: WorkloadStatus::Stopping as i32,
            ..Default::default()
        };
        tx.send(status_stopping).unwrap_or(());


        let status_destroying = InstanceStatus {
            id: workload_id.clone(),
            status: WorkloadStatus::Destroying as i32,
            ..Default::default()
        };

        
        match signal_instruction.signal {
            // Status::Stop
            0 => { 
                let promised = workload.stop();
                tx.send(status_destroying).unwrap_or(());
                promised.await
            },
            // Status::Kill
            1 => {
                let promised = workload.kill();
                tx.send(status_destroying).unwrap_or(());
                promised.await
            }
            _ => return Err(Status::not_found("This signal does not exist")) 
        };


        self.listeners.remove(&workload_id.clone());
        self.workloads.remove(&workload_id);

        Ok(())
    } 
}

impl ToString for WorkloadManager {
    fn to_string(&self) -> String {
        format!("workloads: {}, listeners: {}", self.workloads.len(), self.listeners.len())
    }
}