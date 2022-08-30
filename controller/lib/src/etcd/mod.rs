use etcd_client::{Client, DeleteResponse, GetOptions, PutResponse};
use log::{debug, trace};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EtcdClientError {
    #[error("Etcd error: {0}")]
    EtcdError(#[from] etcd_client::Error),
}

pub struct EtcdClient {
    inner: Client,
}

impl EtcdClient {
    pub async fn new(address: String) -> Result<Self, EtcdClientError> {
        debug!("Starting etcd client on {}", address);

        let inner = Client::connect([address], None)
            .await
            .map_err(EtcdClientError::EtcdError)?;

        Ok(Self { inner })
    }

    pub async fn get(&mut self, key: &str) -> Option<String> {
        if let Ok(response) = self.inner.get(key, None).await {
            if let Some(first) = response.kvs().first() {
                let str = first.value_str().ok().map(|s| s.to_string());

                trace!(
                    "GET key: {}, value: {}",
                    key,
                    str.clone().unwrap_or_default()
                );
                str
            } else {
                trace!("GET key: {}, value: None", key);
                None
            }
        } else {
            trace!("GET key: {}, value: None", key);
            None
        }
    }
    pub async fn put(&mut self, key: &str, value: &str) -> Result<PutResponse, EtcdClientError> {
        trace!("PUT key: {}, value: {}", key, value);
        self.inner.put(key, value, None).await.map_err(|err| {
            let err = EtcdClientError::EtcdError(err);
            trace!("PUT key: {}, error: {}", key, err);
            err
        })
    }
    pub async fn delete(&mut self, key: &str) -> Option<DeleteResponse> {
        match self.get(key).await {
            Some(_) => match self.inner.delete(key, None).await {
                Ok(response) => {
                    trace!("DELETE key: {}, response: {:?}", key, response);
                    Some(response)
                }
                Err(_) => {
                    trace!("DELETE key: {}, response: None", key);
                    None
                }
            },
            None => {
                trace!("DELETE key: {}, response: None", key);
                None
            }
        }
    }

    pub async fn get_all(&mut self) -> Option<Vec<String>> {
        trace!("Getting all keys");

        let resp = self
            .inner
            .get("", Some(GetOptions::new().with_all_keys()))
            .await
            .ok();

        resp.map(|res| {
            let mut values: Vec<String> = vec![];
            for kv in res.kvs() {
                if let Ok(value) = kv.value_str() {
                    values.push(value.to_string());
                }
            }
            values
        })
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
