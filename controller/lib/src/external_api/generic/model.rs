use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Pagination {
    pub limit: u32,
    pub offset: u32,
}

#[derive(Deserialize, Serialize, Default)]
pub struct APIResponse<T> {
    pub data: T,
    pub metadata: APIResponseMetadata,
}

#[derive(Deserialize, Serialize, Default)]
pub struct APIResponseMetadata {
    pub error: Option<String>,
    pub message: Option<String>,
    pub count: Option<u32>,
}
