use etcd_client::{Client, Error, PutResponse, DeleteResponse, GetOptions };
use log::{info,error};

pub struct PaginateRequest {
  limit: usize,
  page: usize
}

impl PaginateRequest {
  pub fn new(limit: usize, page: usize) -> Self {
    PaginateRequest {
      limit: limit,
      page: page
    }
  }
}

pub struct PaginateResult {
  pub count: usize,
  pub values: Vec<String>
}

impl PaginateResult {
  pub fn new(count: usize) -> Self {
    PaginateResult {
      count: count, 
      values: vec![]
    }
  }

  pub fn push(&mut self, value: String) {
    self.values.push(value);
  }

  pub fn get_values(& self) -> &Vec<String> {
    &self.values
  }

  pub fn get_count(& self) -> usize {
    self.count
  }

}

pub struct EtcdInterface {
  client: Client,
}

impl EtcdInterface {
  pub async fn new(address: String) -> Self {
    info!("Starting ETCD client on {}", address);
    EtcdInterface {
        client: Client::connect([address], None).await.unwrap(),
    }
  }

  pub async fn get(&mut self, key: &str) -> Result<String, Error> {
    if let Some(kv) = self.client.get(key, None).await?.kvs().first() {
      let res = kv.value_str();
      info!("Retrieving value in ETCD : Key \"{}\" is associated with value \"{}\"", key, res.as_ref().unwrap());
      res.map(|s| s.to_string())
    } else {
      error!("Error while retrieving value in ETCD : Key \"{}\" not found", key);
      Err(Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found")))
    }
  }
  
  pub async fn put(&mut self, key: &str, value: &str) -> Result<PutResponse, Error> {
    info!("Inserting value in ETCD : Key \"{}\" associated with value \"{}\"", key, value);
    self.client.put(key, value, None).await
  }
  
  pub async fn patch(&mut self, key: &str, value: &str) -> Result<PutResponse, Error> {
    info!("Updating value in ETCD : Key \"{}\" associated with new value \"{}\"", key, value);
    self.client.put(key, value, None).await
  }
  
  pub async fn delete(&mut self, key: &str) -> Result<DeleteResponse, Error> {
    if let Some(kv) = self.client.get(key, None).await?.kvs().first() {
      info!("Deleting value in ETCD : Key \"{}\" ", key);
      self.client.delete(key, None).await
    } else {
      error!("Error while deleting value in ETCD : Key \"{}\" not found", key);
      Err(Error::from(std::io::Error::new(std::io::ErrorKind::NotFound, "Key not found")))
    }
  }

  pub async fn get_all(&mut self) -> Result<Vec<String>,Error> {
    info!("Retrieving all keys in ETCD");
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

  pub async fn get_all_pagination(&mut self, paginate_request: PaginateRequest) -> Result<PaginateResult,Error> {
    let resp = self.client
    .get("", Some(GetOptions::new().with_all_keys()))
    .await?;


    let length = resp.kvs().len();
    
    // Avoid error with negative index
    let mut page = paginate_request.page;
    if page == 0 {
      page= 1;
    }

    let first_index: usize = page * paginate_request.limit - paginate_request.limit;
    let max_index_in_request:usize = first_index + paginate_request.limit;
    let max_index_in_keys = if length < max_index_in_request {length} else {max_index_in_request};  
    
    let mut paginate_result: PaginateResult = PaginateResult::new(length);

    for i in first_index..max_index_in_keys {
      let value = resp.kvs()[i].value_str()?;
      paginate_result.push(value.to_string());
    }
    info!("Retrieving all keys with pagination in ETCD ({} showed/ {} total)", paginate_result.values.len(), length);
    Ok(paginate_result)

  }
}

#[cfg(test)]
mod tests {

  use etcd_client::{Error};
  use crate::internal_etcd::interface::EtcdInterface;
  use crate::internal_etcd::interface::PaginateRequest;

  #[tokio::test]
  async fn test_value_insertion() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("foo", "bar").await?;
    let resp = etcd_client.get("foo").await?;
    assert_eq!(resp, "bar");
    Ok(())
  }

  #[tokio::test]
  async fn test_value_modification() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new("localhost:2379".to_string()).await;
    etcd_client.put("foo", "bar").await?;
    let _res = etcd_client.patch("foo", "baz").await?;
    let resp = etcd_client.get("foo").await?;
    assert_eq!(resp, "baz");
    Ok(())
  }
  
  #[tokio::test]
  async fn test_value_deletion() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("foo", "bar").await?;
    let _res = etcd_client.delete("foo").await?;
    let err = etcd_client.get("foo").await;
    assert!(err.is_err());
    Ok(())
  }
  
  #[tokio::test]
  async fn test_value_deletion_doesnt_exists() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("foo", "bar").await?;
    let err = etcd_client.delete("foo2").await;
    assert!(err.is_err());
    Ok(())
  }

  #[tokio::test]
  async fn test_function_get_all() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("bar", "foo").await;
    let _res = etcd_client.put("hello", "world").await;
    let values = etcd_client.get_all().await?;
    assert_eq!(values[0], "foo");
    assert_eq!(values[1], "world");
    Ok(())
  }

  #[tokio::test]
  async fn test_function_get_all_pagination() -> Result<(), Error> {
    let mut etcd_client = EtcdInterface::new("localhost:2379".to_string()).await;
    let _res = etcd_client.put("bar", "foo").await;
    let _res = etcd_client.put("foo", "bar").await;
    let _res = etcd_client.put("hello", "world").await;
    let paginate_result = etcd_client.get_all_pagination(PaginateRequest::new(5, 1)).await?;
    assert_eq!(paginate_result.count, 3);
    assert_eq!(paginate_result.values[0], "foo");
    assert_eq!(paginate_result.values[1], "bar");
    assert_eq!(paginate_result.values[2], "world");
    Ok(())
  }

}