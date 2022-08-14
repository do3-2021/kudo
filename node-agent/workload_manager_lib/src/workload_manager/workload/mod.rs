use rand::{Rng}; //mocking container id

pub struct Workload {
    id: String
} //mock for now
impl Workload {
    pub fn stop(&self) {
        println!("[MOCK], stopping {}", self.id);
    }
    
    pub fn kill(&self) {
        println!("[MOCK], killing {}", self.id);
    }

    pub fn getId(&self) -> String {
        String::from(&self.id)
    }

    pub fn create() -> Self {
        let mut rng = rand::thread_rng();
        Workload{id: format!("{}", rng.gen_range(0..666))}
    }
}

pub mod error;