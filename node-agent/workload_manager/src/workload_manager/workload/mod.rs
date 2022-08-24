use rand::{Rng}; //mocking container id
use proto::agent::Instance;

pub struct Workload {
    id: String
} //mock for now
impl Workload {
    pub async fn stop(&self) {
        println!("[MOCK], stopping {}", self.id);
    }
    
    pub async fn kill(&self) {
        println!("[MOCK], killing {}", self.id);
    }

    pub fn get_id(&self) -> String {
        String::from(&self.id)
    }

    pub async fn create(instance: Instance) -> Self {
        let mut rng = rand::thread_rng();
        Workload{id: format!("{}", rng.gen_range(0..666))}
    }
}

pub mod error;