use etcd_client::{Client, PutResponse, DeleteResponse, Error, GetOptions, DeleteOptions};
use std::vec;


pub struct EtcdInterface {
  client: Client,
}

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
  count: usize,
  values: Vec<String>
}

impl PaginateResult {
  pub fn new() -> Self {
    PaginateResult {
      count: 0, 
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

  pub fn set_count(&mut self, count: usize) {
    self.count = count;
  }
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
    
    let mut paginate_result: PaginateResult = PaginateResult::new();
    paginate_result.set_count(length);

    for i in first_index..max_index_in_keys {
      let value = resp.kvs()[i].value_str()?;
      paginate_result.push(value.to_string());
    }
    
    Ok(paginate_result)

  }


}