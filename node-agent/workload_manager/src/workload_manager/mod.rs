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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;


    #[test]
    fn constructor() {
        let wm = super::WorkloadManager::new();
        
        assert_eq!(wm.listeners.capacity(), 0);
        assert_eq!(wm.workloads.capacity(), 0);
    }

    #[tokio::test]
    async fn create() {
        let mut wm = super::WorkloadManager::new();
        
        let instance = proto::agent::Instance {
            id: "someuuid".to_string(),
            name: "somename".to_string(),
            r#type: proto::agent::Type::Container as i32,
            status: proto::agent::Status::Running as i32,
            uri: "debian:latest".to_string(),
            environment: vec!["A=0".to_string()],
            resource: Some(proto::agent::Resource {
                limit: Some(proto::agent::ResourceSummary {
                    cpu: i32::MAX,
                    memory: i32::MAX,
                    disk: i32::MAX,
                }),
                usage: Some(proto::agent::ResourceSummary {
                    cpu: 0,
                    memory: 0,
                    disk: 0,
                }),
            }),
            ports: vec![],
            ip: "127.0.0.1".to_string(),
        };

        let rx = wm.create(instance).await.unwrap();

        // uncomment this when workloads will be merged
        // let received = rx.recv().unwrap();
        // assert!(received.resource.unwrap().usage.unwrap().cpu >= 0);

        println!("{:?}", wm.workloads.keys());
        assert_eq!(wm.listeners.len(), 0);
        assert_eq!(wm.workloads.len(), 0);

    }

    #[tokio::test]
    async fn signal() {
        let mut wm = super::WorkloadManager::new();
        
        let instance = proto::agent::Instance {
            id: "someuuid".to_string(),
            name: "somename".to_string(),
            r#type: proto::agent::Type::Container as i32,
            status: proto::agent::Status::Running as i32,
            uri: "debian:latest".to_string(),
            environment: vec!["A=0".to_string()],
            resource: Some(proto::agent::Resource {
                limit: Some(proto::agent::ResourceSummary {
                    cpu: i32::MAX,
                    memory: i32::MAX,
                    disk: i32::MAX,
                }),
                usage: Some(proto::agent::ResourceSummary {
                    cpu: 0,
                    memory: 0,
                    disk: 0,
                }),
            }),
            ports: vec![],
            ip: "127.0.0.1".to_string(),
        };

        let signal = proto::agent::SignalInstruction{
            instance: Some(proto::agent::Instance {
                id: "someuuid".to_string(),
                name: "somename".to_string(),
                r#type: proto::agent::Type::Container as i32,
                ..Default::default()
            }),
            signal: proto::agent::Signal::Kill as i32
        };

        let tx = wm.tx_rx.0.clone();
        let rx = wm.create(instance).await.unwrap();
        
        let (_to_replace, not_used_rx ) = std::sync::mpsc::channel();

        let hshmpwrkld: HashMap<String, crate::workload_manager::Workload> = std::collections::HashMap::new();
        let hshmplstn: HashMap<String, crate::workload_manager::workload_listener::WorkloadListener> = std::collections::HashMap::new();


        // cannot borrow `wm` as mutable more than once at a timesecond mutable borrow occurs here
        let mut wm2 = crate::workload_manager::WorkloadManager { 
            workloads: hshmpwrkld,
            listeners: hshmplstn,
            tx_rx: (tx, not_used_rx)
        };
        
        wm2.signal(signal).await.unwrap();
        
        for _ in 0..2 {
            let recv = rx.recv().unwrap();
            assert!(
                recv.status == proto::agent::Status::Stopping as i32
                ||
                recv.status == proto::agent::Status::Destroying as i32
            );
        }

        assert_eq!(wm.listeners.len(), 0);

    }

}