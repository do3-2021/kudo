use etcd_client::{Client, PutResponse, DeleteResponse, Error, GetOptions};
use std::vec;

pub struct EtcdInterface {
  client: Client,
}

impl EtcdInterface {
  pub async fn new() -> Self {
    EtcdInterface {
        client: Client::connect(["localhost:2379"], None).await.unwrap(),
    }
  }

  pub async fn get(&mut self, key: &str) -> Result<String, Error> {
    if let Some(kv) = self.client.get(key, None).await?.kvs().first() {
      let res = kv.value_str();
      res.map(|s| s.to_string())
    } else {
      Err(Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found")))
    }

  }

  pub async fn get_all(&mut self) -> Result<Vec<String>,Error> {

    let resp = self.client
        .get("", Some(GetOptions::new().with_all_keys()))
        .await?;

    let mut values: Vec<String> = vec![];
    for kv in resp.kvs() {
      let value = kv.value_str()?;
      values.push(value.to_string())
    }
    Ok(values)
  }

  pub async fn put(&mut self, key: &str, value: &str) -> Result<PutResponse, Error> {
    self.client.put(key, value, None).await
  }

  pub async fn patch(&mut self, key: &str, value: &str) -> Result<PutResponse, Error> {
    self.client.put(key, value, None).await
  }

  pub async fn delete(&mut self, key: &str) -> Result<DeleteResponse, Error> {
    self.client.delete(key, None).await
  }


}