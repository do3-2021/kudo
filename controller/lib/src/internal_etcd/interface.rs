use etcd_client::{Client};

pub struct EtcdInterface {
  client: Client,
}

impl EtcdInterface {
  pub async fn new() -> Self {
    EtcdInterface {
        client: Client::connect(["localhost:2379"], None).await.unwrap(),
    }
  }
}