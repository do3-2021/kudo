use etcd_client::{Client, DeleteResponse, Error, GetOptions, PutResponse};
use log::info;

pub struct EtcdClient {
    inner: Client,
}

impl EtcdClient {
    pub async fn new(address: String) -> Result<Self, Error> {
        let inner = Client::connect([address], None).await?;
        Ok(Self { inner })
    }

    pub async fn get(&mut self, key: &str) -> Option<String> {
        match self.inner.get(key, None).await {
            Ok(response) => match response.kvs().first() {
                Some(kv) => match kv.value_str() {
                    Ok(value) => Some(value.to_string()),
                    Err(_) => None,
                },
                None => None,
            },
            Err(_) => None,
        }
    }
    pub async fn put(&mut self, key: &str, value: &str) -> Result<PutResponse, Error> {
        info!(
            "Inserting value in ETCD : Key \"{}\" associated with value \"{}\"",
            key, value
        );
        self.inner.put(key, value, None).await
    }
    pub async fn delete(&mut self, key: &str) -> Option<DeleteResponse> {
        match self.get(key).await {
            Some(_) => match self.inner.delete(key, None).await {
                Ok(response) => Some(response),
                Err(_) => None,
            },
            None => None,
        }
    }

    pub async fn get_all(&mut self) -> Result<Vec<String>, Error> {
        info!("Retrieving all keys in ETCD");
        let resp = self
            .inner
            .get("", Some(GetOptions::new().with_all_keys()))
            .await?;

        let mut values: Vec<String> = vec![];
        for kv in resp.kvs() {
            let value = kv.value_str()?;
            values.push(value.to_string())
        }
        Ok(values)
    }
}
/*

#[cfg(test)]
mod tests {

  use crate::etcd::EtcdClient;
  use etcd_client::Error;

  #[tokio::test]
  async fn test_value_insertion() -> Result<(), Error> {
    let mut etcd_client = EtcdClient::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("foo", "bar").await?;
    let resp = etcd_client.get("foo").await?;
    assert_eq!(resp, "bar");
    Ok(())
  }

  #[tokio::test]
  async fn test_value_modification() -> Result<(), Error> {
    let mut etcd_client = EtcdClient::new("localhost:2379".to_string()).await;
    etcd_client.put("foo", "bar").await?;
    let _res = etcd_client.patch("foo", "baz").await?;
    let resp = etcd_client.get("foo").await?;
    assert_eq!(resp, "baz");
    Ok(())
  }
  #[tokio::test]
  async fn test_value_deletion() -> Result<(), Error> {
    let mut etcd_client = EtcdClient::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("foo", "bar").await?;
    let _res = etcd_client.delete("foo").await?;
    let err = etcd_client.get("foo").await;
    assert!(err.is_err());
    Ok(())
  }
  #[tokio::test]
  async fn test_value_deletion_doesnt_exists() -> Result<(), Error> {
    let mut etcd_client = EtcdClient::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("foo", "bar").await?;
    let err = etcd_client.delete("foo2").await;
    assert!(err.is_err());
    Ok(())
  }

  #[tokio::test]
  async fn test_function_get_all() -> Result<(), Error> {
    let mut etcd_client = EtcdClient::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("bar", "foo").await;
    let _res = etcd_client.put("hello", "world").await;
    let values = etcd_client.get_all().await?;
    assert_eq!(values[0], "foo");
    assert_eq!(values[1], "world");
    Ok(())
  }
}
*/
