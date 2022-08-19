use etcd_client::{Client, Error, PutResponse, DeleteResponse };

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

#[cfg(test)]
mod tests {

  use etcd_client::{Error};
  use crate::internal_etcd::interface::EtcdInterface;

  #[tokio::test]
  async fn test_value_insertion() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new().await;
    let _res = etcd_client.put("foo", "bar").await?;
    let resp = etcd_client.get("foo").await?;
    assert_eq!(resp, "bar");
    Ok(())
  }

  #[tokio::test]
  async fn test_value_modification() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new().await;
    etcd_client.put("foo", "bar").await?;
    let _res = etcd_client.patch("foo", "baz").await?;
    let resp = etcd_client.get("foo").await?;
    assert_eq!(resp, "baz");
    Ok(())
  }
  
  #[tokio::test]
  async fn test_value_deletion() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new().await;
    let _res = etcd_client.put("foo", "bar").await?;
    let _res = etcd_client.delete("foo").await?;
    let err = etcd_client.get("foo").await;
    assert!(err.is_err());
    Ok(())
  }
  
  #[tokio::test]
  async fn test_value_deletion_doesnt_exists() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new().await;
    let _res = etcd_client.put("foo", "bar").await?;
    let err = etcd_client.delete("foo2").await;
    assert!(err.is_err());
    Ok(())
  }

}